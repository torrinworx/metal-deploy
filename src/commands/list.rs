use std::fs;

/*
List all services, online or offline.

list with command -l for live will continuously show updates about the services status
and other information like cpu/memory usage as well as uptime, network up/down, etc. just
cool stats n stuff that could be useful.
*/
pub fn run() {
    // Path to the home directory
    let home_dir = "/home";

    match fs::read_dir(home_dir) {
        Ok(entries) => {
            println!("List of services:");
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let config_file_path = path.join("metal-deploy.config.json");

                    if config_file_path.exists() {
                        if let Some(service_name) = path.file_name().and_then(|name| name.to_str())
                        {
                            println!("{}", service_name);
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to read /home directory: {}", e),
    }
}
