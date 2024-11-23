use std::process::Command;
use std::io::{self, Write};
use regex::Regex;

//
/*
Add service to system.

Creates a new user, clones the git repo, 
*/
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
        println!("User {} already exists. Adding the service again will wipe and overwrite the existing service's data entirely.", service_name);
        print!("Do you want to continue? (yes/no): ");

        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() != "yes" {
            println!("Operation aborted.");
            return;
        }

        Command::new("sudo")
            .arg("userdel")
            .arg("-r")
            .arg(&service_name)
            .output()
            .expect("Failed to delete existing user");
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
        eprintln!("Failed to create user: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn clean_name(raw_name: String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    re.replace_all(&raw_name, "_").to_lowercase()
}
