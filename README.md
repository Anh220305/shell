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

## Architecture

The shell follows a simple Read-Parse-Execute loop:

1. **Read**: Read command from stdin
2. **Parse**: Split the command into program and arguments
3. **Execute**: Run the command (either builtin or external process)

### Key Components

- `rsh_loop()`: Main shell loop
- `rsh_read_line()`: Read input from user
- `rsh_split_line()`: Parse command into arguments
- `rsh_execute()`: Determine if command is builtin or external
- `rsh_launch()`: Execute external programs
- Builtin functions: `rsh_cd()`, `rsh_help()`, `rsh_exit()`

## Enhanced Version

The enhanced shell (`--enhanced` flag) includes additional features:

- Environment variable expansion (`$VAR`)
- Command history (`history` command)
- Additional built-ins: `pwd`, `echo`, `set`, `unset`, `env`, `which`
- Smart prompt showing current directory
- Enhanced error handling

## Differences from C Version

- Uses Rust's memory safety features (no manual memory management)
- Leverages `std::process::Command` for process spawning
- Uses `Vec<String>` instead of `char**` for arguments
- Automatic string handling and UTF-8 support
- Built-in error handling with `Result` types

## Limitations

- No piping or redirection
- No command history (basic version)
- No tab completion
- No background processes
- No environment variable expansion (basic version)
- No globbing (wildcard expansion)
- No quoting or escape sequences

## Future Enhancements

- Command history with arrow key navigation
- Tab completion
- Piping and redirection (`|`, `>`, `<`)
- Background processes (`&`)
- Environment variable expansion (`$VAR`)
- Globbing (`*.txt`)
- Configuration file support
- Job control 