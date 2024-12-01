use std::process::Command;
use std::fs;

/// Delete an existing service
pub fn run(service_name: String) {
	println!("Deleting service: {}", service_name);

	// Stop the service using systemctl within the user's context
	stop_service(&service_name);

	// Remove the systemd service configuration if it exists
	let service_path = format!("/home/{}/.config/systemd", service_name);
	if let Err(e) = fs::remove_dir_all(&service_path) {
		eprintln!("Failed to remove systemd service configuration: {:?}", e);
	} else {
		println!("Removed systemd service configuration.");
	}

	// Kill any remaining processes owned by the user
	kill_user_processes(&service_name);

	// Wait for processes to terminate
	wait_for_processes(&service_name);

	// Delete the service user
	let userdel_status = Command::new("sudo")
		.arg("userdel")
		.arg("-r")
		.arg(&service_name)
		.status()
		.expect("Failed to execute userdel command");
	
	if userdel_status.success() {
		println!("User '{}' and their home directory removed successfully.", service_name);
	} else {
		eprintln!("Error: Failed to remove user '{}'.", service_name);
	}

	println!("Service '{}' fully deleted.", service_name);
}

/// Function to stop a running service within the user's context
fn stop_service(service_name: &str) {
	use crate::utils::systemctl::systemctl;

	println!("Attempting to stop the service: {}", service_name);
	systemctl(service_name, "stop");
}

/// Function to kill processes owned by the user
fn kill_user_processes(service_name: &str) {
	println!("Killing any remaining processes for user: {}", service_name);

	let killall_status = Command::new("sudo")
		.arg("killall")
		.arg("-9")
		.arg("--user")
		.arg(service_name)
		.status()
		.expect("Failed to execute killall command");

	if !killall_status.success() {
		eprintln!("Warning: Could not kill all processes for user '{}'.", service_name);
	}
}

/// Function to wait for processes to terminate
fn wait_for_processes(service_name: &str) {
    loop {
        let check_status = Command::new("pgrep")
            .arg("-u")
            .arg(service_name)
            .status()
            .expect("Failed to execute pgrep command");

        if !check_status.success() {
            println!("All processes for user '{}' have been terminated.", service_name);
            break;
        }

        // Optional sleep to reduce CPU usage while waiting
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
