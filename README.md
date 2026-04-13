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
* Linux (this program require `/dev/urandom` and libc from `glibc` or `musl`).
  These are the known distribution to support out of the box (by just install
  `cargo` package):
    * Debian 13 Trixie
    * Fedora 43
    * RedHat Enterprise Linux or compatible distros (tested on Rocky Linux):
        * EL8 (requires `rust-toolset` DNF module to be enabled first)
        * EL9
        * EL10
    * Ubuntu 25.10

For Linux distribution that doesn't come with Rust compiler that supports 2024
edition, remove the Rust toolchain from your Linux distro and install rust with
[rustup.rs](https://rustup.rs/) instead.

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
