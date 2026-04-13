use clap::Parser;
use signal_notify::{Signal, notify};
use std::thread;

mod memory_hogger {
    use std::{fs, io::Read};

    fn get_rand_bytes(s: usize) -> Vec<u8> {
        let mut f = fs::File::open("/dev/urandom").unwrap();
        let mut buffer = vec![0; s];
        f.read_exact(&mut buffer).unwrap();
        buffer
    }

    #[test]
    fn test_get_rand_bytes() {
        const SIZE: usize = 1024;
        let result = get_rand_bytes(SIZE);
        assert_eq!(result.len(), SIZE);
    }

    fn allocate_empty(s: usize) -> Vec<u8> {
        let buffer = vec![0; s];
        buffer
    }

    #[test]
    fn test_allocate_empty() {
        const SIZE: usize = 1024;
        let result = allocate_empty(SIZE);
        assert_eq!(result.len(), SIZE);
    }

    pub fn thread_worker(size: usize, count: usize, random_value: bool) -> Vec<Vec<u8>> {
        let mut hogged = vec![];

        #[cfg(debug_assertions)]
        println!("Thread block count: {count}");

        for _ in 0..count {
            if random_value {
                hogged.push(get_rand_bytes(size));
            } else {
                hogged.push(allocate_empty(size))
            }
        }
        hogged
    }

    pub fn get_hogged_size(hogged: &Vec<Vec<u8>>) -> usize {
        let mut hogged_size = 0;
        for block_ref in hogged {
            let block_size = std::mem::size_of_val(&**block_ref);
            hogged_size += block_size;
            hogged_size += std::mem::size_of_val(block_ref);
        }
        hogged_size += std::mem::size_of_val(hogged);

        hogged_size
    }
}

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

fn main() {
    let args = Args::parse();
    let mut thread_pool = vec![];
    let mut hogged = vec![];

    println!("Block Size:        {}", args.block_size);
    println!("Block Count:       {}", args.block_count);
    println!("Fill Random Value: {}", args.fill_random);
    println!("Threads:           {}", args.threads);

    if args.threads < 1 {
        panic!("--threads cannot be lower than 1");
    }

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

    println!("Memory Hogged");

    let expected_size = args.block_count * args.block_size;
    let hogged_size = memory_hogger::get_hogged_size(&hogged);
    let size_overhead = hogged_size - expected_size;
    println!("Expected Hog Size:  {expected_size} Bytes");
    println!("Actual Hogged Size: {hogged_size} Bytes");
    println!("Overhead:           {size_overhead} Bytes");

    // signal-notify doesn't provide an interface for converting integer into
    // Signal enums
    let signal = notify(&[
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
    match signal.recv() {
        Ok(v) => println!("Received signal {v:?}"),
        Err(e) => println!("Error: {e:?}"),
    }

    println!("Exiting");
}
