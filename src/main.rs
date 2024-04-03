use std::env;

mod update;
mod install;

use crate::update::update_system;
use crate::install::install_package;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if any arguments are provided
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }

    // Check the first argument
    match args[1].as_str() {
        "update" => {
            println!("Running update command...");
            update_system();
        }
        "install" => {
            // Check if the second argument exists
            if args.len() == 3 {
                let package = &args[2];
                println!("Installing package: {}", package);
                match install_package(package) {
                    Ok(()) => println!("Package '{}' installed successfully.", package),
                    Err(err) => eprintln!("Error: {}", err),
                }
                return;
            } else {
                println!("Usage: {} install <package_name>", args[0]);
                return;
            }
        }
        _ => {
            println!("Invalid command: {}", args[1]);
            println!("Usage: {} <command>", args[0]);
        }
    }
}
