# Memory Hogger

This program just hogs a bunch of memory and do nothing about it.

This tool is absolutely useless except you want to create a high memory usage
environment.

Also, this is my first program written in rust.


## Build and Install

To build this program, you'll need:

* Rust compiler >= 1.85.0 (This project requires compilers that supports
  2024 edition rust.)
* Rust cargo package manager

This program is tested in these environment:

* Fedora 43 Workstation
    * `cargo` 1.94.1 from Fedora
* Windows Server 2025 Datacenter with
    * `cargo` 1.94.1 from chocolately
    * MSYS2 package group `mingw-w64-ucrt-x86_64-toolchain` (all packages)
    * MSYS2 `mingw-w64-ucrt-x86_64-nasm`

```shell
#!/bin/bash
cargo install --git https://github.com/jacky9813/memory_hogger
```

> [!NOTE]
> You may need to export cargo installed binaries directory to `PATH` if you
> haven't done so.
>
> ```shell
> #!/bin/bash
> export PATH="$PATH:$HOME/.cargo/bin"
> ```

## Usage

There are 2 requires options:

* `--block-count <INT>` or `-c <INT>`: How many blocks of memory to hog.
* `--block-size <INT>` or `-s <INT>`: How large each memory block is.

> [!NOTE]
> In many systems, only virtual memory will be reserved if no data is written
> to the hogged memory. To resolve this issue, use `--fill-random` or `-r` to
> fill memory from `/dev/urandom`. This will make use of the resident memory,
> which is counted as actual memory usage.

> [!TIP]
> To enable faster memory allocation, use `--threads <INT>` or `-t <INT>` for
> multi-threaded operation. This will have significant performance gains when
> `--fill-random` option is used.

Example:

```shell
# Hog 1 GiB of memory.
memory_hogger -s 1048576 -c 1024 -t 8 -r
```

After the program has finished hogging memory, it'll hang until you send a
signal (like Ctrl-C) to it, which will trigger program exit.

Use `memory_hogger --help` to get all available configurations.


> [!NOTE]
> Memory Hogger will have about `(block count + 1) * 24` Bytes overhead.


> [!IMPORTANT]
> Extremely large block size will be much difficult to reserve, since the
> underlying data structure needs to be continuous virtual memory space.
> If you found weird behavior of Memory Hogger, try decrease the block size and
> increase block count for reserving the same amount of memory space instead.
