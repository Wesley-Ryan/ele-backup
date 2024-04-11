use std::env;

mod update;
mod install;

use crate::update::update_system;
use crate::install::install_package;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "update" => update_system(), // `update_system` directly handles errors and exits.
        "install" => {
            if args.len() < 3 {
                println!("Usage: {} install <package_name>", args[0]);
                return;
            }
            let package = &args[2];
            match install_package(package) {
                Ok(()) => println!("Package '{}' installed successfully.", package),
                Err(err) => eprintln!("Error installing package '{}': {}", package, err),
            }
        }
        _ => print_invalid_command(&args[1], &args[0]),
    }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <command>", program_name);
    println!("Commands:");
    println!("  update                    Updates the system");
    println!("  install <package_name>    Installs the specified package");
}

fn print_invalid_command(command: &str, program_name: &str) {
    println!("Invalid command: {}", command);
    print_usage(program_name);
}
