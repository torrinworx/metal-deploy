use crate::utils::systemctl::systemctl;
use std::path::Path;

/*
Stop a currently running service
*/
pub fn run(service_name: String) {
    println!("Stopping service: {}", service_name);

    let service_dir = format!("/home/{}/.config/systemd/user", service_name);
    let service_file_path = format!("{}/{}.service", service_dir, service_name);

    // Check if the service file exists
    if Path::new(&service_file_path).exists() {
        // Stop the service
        systemctl(&service_name, "stop");
        println!("Service {} has been stopped.", service_name);
    } else {
        println!(
            "No systemd service found for {}, assuming it's already stopped or does not exist.",
            service_name
        );
    }
}
