# Memory Hogger

This program just hogs a bunch of memory and do nothing about it.

This tool is absolutely useless except you want to create a high memory usage
environment.


Also, this is my first program written in rust.


## Install

```shell
#!/bin/bash
cargo install --git github.com/jacky9813/memory-hogger
```

## Usage 

There are 2 requires options:

* `--block-count <INT>` or `-c <INT>`: How many blocks of memory to hog.
* `--block-size <INT>` or `-s <INT>`: How large each memory block is.

> [!NOTE]
> In many systems, only virtual memory will be reserved if no data is written
> to the hogged memory. To resolve this issue, use `--fill-random` or `-r` to
> fill memory from `/dev/urandom`. This will make use of the resident memory,
> which is counted as actual memory usage.

Example:

```shell
# Hog 1 GiB of memory.
memory_hogger -s 1048576 -c 1024 -r
```

After the program has finished hogging memory, it'll hang until you send a
signal (like Ctrl-C) to it, which will trigger program exit.

Use `memory_hogger --help` to get all available configurations.
