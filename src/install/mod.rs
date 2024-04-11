use std::io::{self, ErrorKind};
use std::process::Command;
use std::error::Error;

pub fn install_package(package_name: &str) -> Result<(), Box<dyn Error>> {
    // Create a Timeshift snapshot
    let timeshift_result = Command::new("timeshift")
        .arg("--create")
        .status();

    // Check if Timeshift command was successful
    match timeshift_result {
        Ok(status) => {
            if !status.success() {
                return Err("Timeshift snapshot creation failed. Aborting.".into());
            }
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err("'timeshift' command not found. Make sure it's installed and available in PATH.".into());
            } else {
                return Err(format!("Failed to execute timeshift command: {}", e).into());
            }
        }
    }

    // Prompt the user for confirmation before installing
    println!("Do you want to install '{}'? (y/n): ", package_name);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");

    if answer.trim().eq_ignore_ascii_case("y") {
        // If user confirms, proceed with dnf install
        let dnf_result = Command::new("dnf")
            .arg("install")
            .arg(package_name)
            .arg("-y")
            .status();

        // Check if dnf command was successful
        match dnf_result {
            Ok(status) => {
                if !status.success() {
                    return Err(format!("Failed to install the package '{}'.", package_name).into());
                } else {
                    println!("Package '{}' installed successfully.", package_name);
                }
            },
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Err("'dnf' command not found. Make sure it's installed and available in PATH.".into());
                } else {
                    return Err(format!("Failed to execute dnf command: {}", e).into());
                }
            }
        }
    } else {
        println!("Installation aborted by the user."); // Correctly indicate that the installation was aborted
        return Ok(());
    }

    Ok(())
}
