use crate::utils::systemctl::systemctl;
use std::path::Path;

/*
Restart an existing service that is currently running
*/
pub fn run(service_name: String) {
    println!("Restarting service: {}", service_name);

    let service_dir = format!("/home/{}/.config/systemd/user", service_name);
    let service_file_path = format!("{}/{}.service", service_dir, service_name);

    // Check if the service file exists
    if Path::new(&service_file_path).exists() {
        // Restart the service
        systemctl(&service_name, "restart");
        println!("Service {} has been restarted.", service_name);
    } else {
        println!(
            "No systemd service found for {}, assuming it does not exist.",
            service_name
        );
    }
}
