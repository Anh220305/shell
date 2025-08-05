use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};
use std::path::Path;

mod enhanced;
use enhanced::EnhancedShell;

const BUILTIN_COMMANDS: &[&str] = &["cd", "help", "exit"];

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--enhanced" {
        // Run enhanced shell
        let mut enhanced_shell = EnhancedShell::new();
        enhanced_shell.run();
    } else {
        // Run basic shell
        println!("Basic Rust Shell (RSH)");
        println!("Use --enhanced flag for the enhanced version");
        rsh_loop();
        println!("Shell terminated.");
    }
}

fn rsh_loop() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let line = rsh_read_line();
        let args = rsh_split_line(&line);
        let status = rsh_execute(&args);
        
        if !status {
            break;
        }
    }
}

fn rsh_read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim_end().to_string()
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
            exit(1);
        }
    }
}

fn rsh_split_line(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn rsh_execute(args: &[String]) -> bool {
    if args.is_empty() {
        return true;
    }
    
    match args[0].as_str() {
        "cd" => rsh_cd(args),
        "help" => rsh_help(args),
        "exit" => rsh_exit(args),
        _ => rsh_launch(args),
    }
}

fn rsh_launch(args: &[String]) -> bool {
    let program = &args[0];
    let arguments = &args[1..];
    
    match Command::new(program)
        .args(arguments)
        .status()
    {
        Ok(_status) => {
            true
        }
        Err(error) => {
            eprintln!("rsh: {}: {}", program, error);
            true
        }
    }
}

fn rsh_cd(args: &[String]) -> bool {
    if args.len() < 2 {
        eprintln!("rsh: expected argument to \"cd\"");
    } else {
        let new_dir = &args[1];
        if let Err(error) = env::set_current_dir(Path::new(new_dir)) {
            eprintln!("rsh: cd: {}: {}", new_dir, error);
        }
    }
    true
}

fn rsh_help(_args: &[String]) -> bool {
    println!("Rust Shell (RSH)");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are built in:");
    
    for &builtin in BUILTIN_COMMANDS {
        println!("  {}", builtin);
    }
    
    println!("Use the man command for information on other programs.");
    true
}
fn rsh_exit(_args: &[String]) -> bool {
    false 
} 