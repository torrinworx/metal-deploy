use crate::commands::{build, start, stop};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/*
Update the service to the latest release or latest commits
*/
pub fn run(service_name: String, latest_release: bool) {
    println!("Updating service: {}", service_name);

    let repo_dir = format!("/home/{}/repo", service_name);
    let build_dir = format!("/home/{}/build", service_name);

    // Pull latest changes from the repo
    let mut git_command = Command::new("git");
    git_command.arg("pull");

    if latest_release {
        git_command.arg("--tags");
    }

    git_command
        .current_dir(&repo_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if !git_command
        .status()
        .expect("Failed to execute git pull")
        .success()
    {
        eprintln!(
            "Failed to pull latest changes for service: {}",
            service_name
        );
        return;
    }
    println!(
        "Latest changes pulled successfully for service: {}",
        service_name
    );

    // Build the repo again
    build::run(service_name.clone(), false);

    // Stop the existing service
    stop::run(service_name.clone());

    // Clear or create the /home/service_name/build folder
    if Path::new(&build_dir).exists() {
        if let Err(e) = clear_directory_contents(&build_dir) {
            eprintln!("Error clearing existing build directory: {:?}", e);
            return;
        }
        println!("Existing build directory cleared successfully.");
    } else {
        if let Err(e) = fs::create_dir_all(&build_dir) {
            eprintln!("Error creating build directory: {:?}", e);
            return;
        }
        println!("Build directory created successfully.");
    }

    // Move the new build contents to /home/service_name/build
    let new_build_dir = format!("/home/{}/repo/build", service_name);
    if let Err(e) = move_directory_contents(&new_build_dir, &build_dir) {
        eprintln!("Error moving new build contents: {:?}", e);
        return;
    }
    println!("New build contents moved successfully.");
    start::run(service_name.clone(), true);
    println!("Service '{}' updated successfully.", service_name);
}

fn move_directory_contents(src_dir: &str, dest_dir: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dest_dir).join(entry.file_name());
        fs::rename(&path, &dest_path)?;
    }
    Ok(())
}

fn clear_directory_contents(dir: &str) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
