# Rust Shell (RSH)

A simple Unix shell implementation in Rust, based on Stephen Brennan's "Write a Shell in C" tutorial.

## Features

- Interactive command prompt
- Execute external programs
- Built-in commands:
  - `cd` - Change directory
  - `help` - Show available commands
  - `exit` - Exit the shell
- Error handling for invalid commands and failed operations

## Building and Running

```bash
# Build the project
cargo build

# Run the shell
cargo run

# Run enhanced version
cargo run -- --enhanced
```

## Usage

Once running, you'll see a `> ` prompt where you can enter commands:

```bash
> ls -la
> cd /home/user
> help
> exit
```

Article: https://dongnguyenminhanh.notion.site/Writing-a-Shell-in-Rust-2467e587d8a88078bd7dc19fa6f8c39a?source=copy_link
