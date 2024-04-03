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
                // Run dnf update only if Timeshift was successful
                let dnf_result = Command::new("sudo")
                    .arg("dnf")
                    .arg("update")
                    .status();
                if let Err(e) = dnf_result {
                    eprintln!("Error running dnf update: {}", e);
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
