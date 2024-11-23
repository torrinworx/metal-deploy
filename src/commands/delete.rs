use std::fs;
use std::io::ErrorKind;
use std::process::Command;

/*
Delete a service, if a service is still running ask the user if they're sure they want to delete a running service?
Delete the user folder and all service/config/env files.

Before deleting the service ask them if they would like to delete all data, all env variables, and all service data/content.
*/
pub fn run(service_name: String) {
    let config_path = format!("/home/{}/metal-deploy.config.json", service_name);

    if !fs::metadata(&config_path).is_ok() {
        eprintln!(
            "Error: Cannot delete non metal-deploy user: {}",
            service_name
        );
        return;
    }

    Command::new("sudo")
        .arg("userdel")
        .arg("-r")
        .arg("-f")
        .arg(service_name.clone())
        .status()
        .expect("Failed to delete existing user and directory");

    let home_dir = format!("/home/{}", service_name);

    if fs::metadata(&home_dir).is_ok() {
        match fs::remove_dir_all(&home_dir) {
            Ok(_) => println!("Removed existing directory: {}", home_dir),
            Err(ref e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => {
                eprintln!(
                    "Failed to remove existing directory: {}. Error: {:?}",
                    home_dir, e
                );
            }
        }
    }
}
