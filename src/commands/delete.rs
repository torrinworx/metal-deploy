use crate::utils::confirm::confirm;
use crate::utils::run_systemctl::run_systemctl;

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;


pub fn run(service_name: String) {
    println!("Deleting service: {}", service_name);

    let service_dir = format!("/home/{}/.config/systemd/user", service_name);
    let service_file_path = format!("{}/{}.service", service_dir, service_name);

    if Path::new(&service_file_path).exists() {
        run_systemctl(&service_name, "stop");
        run_systemctl(&service_name, "disable");
        run_systemctl(&service_name, "daemon-reload");
        println!("Service {} is stopped and disabled.", service_name);
    } else {
        println!(
            "No systemd service found for {}, assuming it's already removed.",
            service_name
        );
    }

    kill_lingering_processes(&service_name);

    if confirm("Do you want to delete the user and all associated files?") {
        delete_user_and_home(&service_name);
    } else {
        println!("User deletion aborted.");
    }
}

fn kill_lingering_processes(service_name: &str) {
    let ps_output = Command::new("pgrep")
        .arg("-f")
        .arg(&format!("{}|/home/{}/", service_name, service_name))
        .output()
        .expect("Failed to run pgrep");

    if !ps_output.stdout.is_empty() {
        let pids_output = String::from_utf8_lossy(&ps_output.stdout);
        let pids: Vec<&str> = pids_output.lines().collect();
        for pid in pids {
            Command::new("sudo")
                .arg("kill")
                .arg("-9")
                .arg(pid)
                .status()
                .expect("Failed to kill process");
        }
        println!("Killed remaining processes for service: {}", service_name);
    } else {
        println!("No lingering processes found for service: {}", service_name);
    }
}

fn delete_user_and_home(service_name: &str) {
    let user_del_status = Command::new("sudo")
        .arg("userdel")
        .arg("-r")
        .arg("-f")
        .arg(service_name)
        .status()
        .expect("Failed to delete user");

    if !user_del_status.success() {
        eprintln!("Failed to delete user {}", service_name);
    }

    let home_dir = format!("/home/{}", service_name);

    if fs::metadata(&home_dir).is_ok() {
        match fs::remove_dir_all(&home_dir) {
            Ok(_) => println!("Removed existing directory: {}", home_dir),
            Err(ref e) if e.kind() == ErrorKind::NotFound => (),
            Err(e) => {
                eprintln!(
                    "Failed to remove existing directory: {}. Error: {:?}",
                    home_dir, e
                );
            }
        }
    }

    println!("User {} has been deleted.", service_name);
}
