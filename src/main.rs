use clap::Parser;
use dialoguer::Confirm;
use humansize::{BINARY, format_size};
use std::{error, fmt, thread, time};
use systemstat::{Platform, System};

#[derive(Clone)]
struct AbortedError;

impl fmt::Debug for AbortedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User aborted the operation")
    }
}

impl fmt::Display for AbortedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User aborted the operation")
    }
}

impl error::Error for AbortedError {}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short('c'), long, help("How many blocks to allocate"))]
    block_count: usize,

    #[arg(short('s'), long, help("How large each block is"))]
    block_size: usize,

    #[arg(short('r'), long, help(
        "Fill each block with random values. This may help with ".to_owned() +
        "sparse memory allocation."
    ))]
    fill_random: bool,

    #[arg(short, long, help("How many hoggers (threads)"), default_value_t = 1)]
    threads: usize,
}

#[cfg(target_family = "windows")]
fn wait(msg: &str) {
    use windows::Win32::System::Console;
    use windows_result::BOOL;
    println!("{}", msg);
    unsafe extern "system" fn handler(ctrltype: u32) -> BOOL {
        let event_type_name = match ctrltype {
            Console::CTRL_C_EVENT => "Ctrl-C",
            Console::CTRL_BREAK_EVENT => "Ctrl-Break",
            Console::CTRL_CLOSE_EVENT => "Ctrl-Close",
            Console::CTRL_LOGOFF_EVENT => "Ctrl-Logoff",
            Console::CTRL_SHUTDOWN_EVENT => "Ctrl-Shutdown",
            _ => "Unknown Event",
        };
        println!("Received event: {}", event_type_name);
        BOOL(0)
    }
    unsafe {
        let _ = Console::SetConsoleCtrlHandler(Some(handler), true);
    }
    loop {
        std::thread::sleep(std::time::Duration::from_hours(1u64));
    }
}

#[cfg(target_family = "unix")]
fn wait(msg: &str) {
    use signal_notify::{Signal, notify};
    println!("{}", msg);

    // signal-notify doesn't provide an interface for converting integer into
    // Signal enums
    let signal_receiver = notify(&[
        Signal::HUP,
        Signal::INT,
        Signal::QUIT,
        Signal::ILL,
        Signal::ABRT,
        Signal::FPE,
        Signal::SEGV,
        Signal::PIPE,
        Signal::ALRM,
        Signal::TERM,
        Signal::USR1,
        Signal::USR2,
    ]);
    let signal = signal_receiver.recv().unwrap();
    println!("Received signal {signal:?}");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // args data validation
    if args.threads < 1 {
        panic!("--threads cannot be lower than 1");
    }

    let mut thread_pool = vec![];
    let mut hogged = vec![];
    let value_size = args.block_count * args.block_size;
    let vec_size = std::mem::size_of_val(&hogged);
    let expected_overhead = vec_size * (args.block_count + 1);
    let expected_hog_size = value_size + expected_overhead;
    let sys = System::new();
    let mem_stat_r = sys.memory();

    let free_mem_size = match mem_stat_r {
        Ok(mem_stat) => mem_stat.free.as_u64() as usize,
        Err(_) => {
            println!("Failed to fetch system free memory");
            0_usize
        }
    };

    println!("Block Size:          {}", args.block_size);
    println!("Value Block Count:   {}", args.block_count);
    println!("Total Block Count:   {}", args.block_count + 1);
    println!("Fill Random Value:   {}", args.fill_random);
    println!("Threads:             {}", args.threads);
    println!(
        "Overhead Per Block:  {} Bytes ({})",
        vec_size,
        format_size(vec_size, BINARY)
    );
    println!(
        "Total Value Size:    {} Bytes ({})",
        value_size,
        format_size(value_size, BINARY)
    );
    println!(
        "Expected Overhead:   {} Bytes ({})",
        expected_overhead,
        format_size(expected_overhead, BINARY)
    );
    println!(
        "Expected Total Size: {} Bytes ({})",
        expected_hog_size,
        format_size(expected_hog_size, BINARY)
    );
    println!(
        "System Free Memory:  {} Bytes ({})",
        free_mem_size,
        format_size(free_mem_size, BINARY)
    );

    if free_mem_size < expected_hog_size {
        let prompt_resp = Confirm::new()
            .with_prompt(
                "WARNING: System free memory is lower than expected total memory. Continue?",
            )
            .interact()
            .unwrap_or(false);
        if !prompt_resp {
            return Err(AbortedError {}.into());
        }
    }

    let start_time = time::Instant::now();
    for _i in 0..(args.threads - 1) {
        thread_pool.push(thread::spawn(move || {
            let partitioned_count = args.block_count / args.threads;
            memory_hogger::thread_worker(args.block_size, partitioned_count, args.fill_random)
        }));
        #[cfg(debug_assertions)]
        println!("Thread {_i} spawned");
    }
    thread_pool.push(thread::spawn(move || {
        let partitioned_count = (args.block_count / args.threads) * (args.threads - 1);
        memory_hogger::thread_worker(
            args.block_size,
            args.block_count - partitioned_count,
            args.fill_random,
        )
    }));
    #[cfg(debug_assertions)]
    println!("Last Thread spawned");
    for t in thread_pool {
        let result = t.join().unwrap();
        hogged.extend(result);
    }
    let hog_elapse = start_time.elapsed();

    let hogged_size = memory_hogger::get_hogged_size(&hogged);
    let size_overhead = hogged_size - value_size;
    println!("Hogging Time:        {} seconds", hog_elapse.as_secs_f64());
    println!(
        "Actual Overhead:     {} Bytes ({})",
        size_overhead,
        format_size(size_overhead, BINARY)
    );
    println!(
        "Actual Hogged Size:  {} Bytes ({})",
        hogged_size,
        format_size(hogged_size, BINARY)
    );
    wait("Memory Hogged. Waiting a signal to stop...");
    println!("Exiting");
    Ok(())
}
