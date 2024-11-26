use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/// Build the service by executing its build.sh script.
pub fn run(service_name: String, replace_existing: bool) {
    let build_script_path = format!("/home/{}/repo/build.sh", service_name);
    let build_dir_in_repo = format!("/home/{}/repo/build", service_name);
    let build_dir_in_home = format!("/home/{}/build", service_name);

    // Check if the build.sh file exists
    if !Path::new(&build_script_path).exists() {
        eprintln!(
            "Error: build.sh does not exist for service: {}",
            service_name
        );
        return;
    }

    // If replacing existing build, remove previous build directory if it exists in home directory
    if replace_existing {
        if fs::metadata(&build_dir_in_home).is_ok() {
            if let Err(e) = fs::remove_dir_all(&build_dir_in_home) {
                eprintln!("Error removing previous build directory in home: {:?}", e);
                return;
            }
        }
    }

    // Execute the build.sh script and inherit stdout and stderr
    let mut child = Command::new("bash")
        .arg(&build_script_path)
        .current_dir(format!("/home/{}/repo", service_name))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute build.sh");

    // Wait for the command to complete
    let output = child.wait().expect("Failed to wait on child process");

    if output.success() {
        println!("Service {} built successfully.", service_name);

        // After successful build, move the build directory to home directory if replacing existing
        if replace_existing {
            if let Err(e) = fs::rename(&build_dir_in_repo, &build_dir_in_home) {
                eprintln!("Error moving build directory to home: {:?}", e);
            } else {
                println!("Build directory moved to home successfully.");
            }
        }
    } else {
        eprintln!("Failed to build service: {}", service_name);
    }
}
