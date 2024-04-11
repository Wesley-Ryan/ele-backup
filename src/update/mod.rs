use std::io;
use std::process::Command;

pub fn update_system() {
    // Create a Timeshift snapshot
    let timeshift_result = Command::new("timeshift")
        .arg("--create")
        .status();

    // Check if Timeshift command was successful
    match timeshift_result {
        Ok(status) => {
            if status.success() {
                println!("Timeshift snapshot created successfully.");

                // Ask the user if they want to proceed with dnf update
                println!("Do you want to proceed with dnf update? (y/n): ");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer).expect("Failed to read line");
                if answer.trim().eq_ignore_ascii_case("y") {
                    // Run dnf update only if the user confirms
                    let dnf_result = Command::new("sudo")
                        .arg("dnf")
                        .arg("update")
                        .status();
                    if let Err(e) = dnf_result {
                        eprintln!("Error running dnf update: {}", e);
                    }
                } else {
                    println!("dnf update aborted by user.");
                }
            } else {
                eprintln!("Timeshift snapshot creation failed. Aborting.");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error running timeshift: {}", e);
            std::process::exit(1);
        }
    }
}
