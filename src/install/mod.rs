use std::process::{Command};
use std::io::ErrorKind;
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

    // Run dnf install
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

    Ok(())
}
