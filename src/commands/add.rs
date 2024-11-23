use crate::commands::delete;
use crate::utils::confirm::confirm;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// Add service to system.
//
// Creates a new user, clones the git repo,
pub fn run(repo_url: String, name: Option<String>) {
    let service_name = match name {
        Some(custom_name) => clean_name(custom_name),
        None => {
            let parts: Vec<&str> = repo_url.split('/').collect();
            let last_part = parts.last().unwrap_or(&"").to_string();
            let raw_name = last_part.trim_end_matches(".git").to_string();
            clean_name(raw_name)
        }
    };

    println!("Adding service: {} from: {}", service_name, repo_url);

    let user_exists = Command::new("id")
        .arg(&service_name)
        .output()
        .expect("Failed to execute id command")
        .status
        .success();

    if user_exists {
        println!(
			"User {} already exists. Adding the service again will wipe and overwrite the existing service's data entirely.",
			service_name
		);

        if !confirm("Do you want to continue?") {
            println!("Operation aborted.");
            return;
        }

        // Use the delete module's run function
        delete::run(service_name.clone());
    }

    let output = Command::new("sudo")
        .arg("useradd")
        .arg("-m")
        .arg(&service_name)
        .output()
        .expect("Failed to execute useradd command");

    if output.status.success() {
        println!("User {} created successfully.", service_name);
    } else {
        eprintln!(
            "Failed to create user: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    let home_dir = format!("/home/{}/repo", service_name);
    let clone_status = Command::new("git")
        .arg("clone")
        .arg(&repo_url)
        .arg(&home_dir)
        .status()
        .expect("Failed to execute git clone");

    if clone_status.success() {
        println!("Repository cloned successfully into {}", home_dir);

        // Create the `metal-deploy.config.json` file
        let config_path = format!("/home/{}/metal-deploy.config.json", service_name);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let config_content = format!(
            "{{
	\"name\": \"{}\",
	\"url\": \"{}\",
	\"added\": \"{}\"
}}",
            service_name, repo_url, now
        );
        let mut file = File::create(&config_path).expect("Failed to create config file");
        file.write_all(config_content.as_bytes())
            .expect("Failed to write to config file");
        println!("Service configuration file created at: {}", config_path);

        // Check if `build.sh` exists
        let build_script_path = format!("{}/build.sh", home_dir);
        if !fs::metadata(&build_script_path).is_ok() {
            eprintln!("Service has an invalid repo structure: missing build.sh");
            return;
        }
        println!("Service created successfully: {}", service_name);
    } else {
        eprintln!("Failed to clone repository.");
    }
}

fn clean_name(raw_name: String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    re.replace_all(&raw_name, "_").to_lowercase()
}
