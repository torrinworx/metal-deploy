use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Write};
use std::os::unix::fs::symlink;
use std::path::Path;

use crate::utils::systemctl::systemctl;

// Start a service by creating an .env file, a systemd service, and executing the run.sh script.
pub fn run(service_name: String) {
    println!("Starting service: {}", service_name);

    let home_dir = format!("/home/{}", service_name);
    let env_template_path = format!("{}/repo/.metal-deploy.env", home_dir);
    let env_path = format!("{}/.env", home_dir);
    let run_script_path = format!("{}/build/run.sh", home_dir);
    let service_dir = format!("{}/.config/systemd/user", home_dir);
    let service_file_path = format!("{}/{}.service", service_dir, service_name);

    // Check if the .metal-deploy.env file exists
    eprintln!("{}", env_template_path);
    if !Path::new(&env_template_path).exists() {
        eprintln!(
            "Error: .metal-deploy.env does not exist for service: {}",
            service_name
        );
        return;
    }

    // Read the environment template and populate fields
    let env_variables = read_and_populate_env(&env_template_path);
    write_env_to_file(&env_path, &env_variables);

    // Check if the run.sh file exists
    if !Path::new(&run_script_path).exists() {
        eprintln!("Error: run.sh does not exist for service: {}", service_name);
        return;
    }

    // Ensure the service directory exists
    std::fs::create_dir_all(&service_dir).expect("Failed to create service directory");

    // Create the systemd service file
    create_service_file(
        &service_file_path,
        &run_script_path,
        &env_path,
        &service_name,
    );

    systemctl(&service_name, "daemon-reload");
    systemctl(&service_name, "enable");
    systemctl(&service_name, "start");

    println!(
        "Service {} has been created and started for user {}",
        service_name, service_name
    );
}

fn read_and_populate_env(template_path: &str) -> HashMap<String, String> {
    let mut env_map = HashMap::new();
    let file = File::open(template_path).expect("Failed to open .metal-deploy.env");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let value = if value.is_empty() {
                print!("Enter value for {}: ", key);
                io::stdout().flush().expect("Failed to flush stdout");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read user input");
                input.trim().to_string()
            } else {
                value.to_string()
            };
            env_map.insert(key.to_string(), value);
        }
    }

    env_map
}

fn write_env_to_file(env_path: &str, env_vars: &HashMap<String, String>) {
    let mut file = File::create(env_path).expect("Failed to create .env file");

    for (key, value) in env_vars {
        writeln!(file, "{}={}", key, value).expect("Failed to write to .env file");
    }

    println!(".env file created at: {}", env_path);
}

fn create_service_file(
    service_file_path: &str,
    run_script_path: &str,
    env_path: &str,
    service_name: &str,
) {
    let service_content = format!(
        "[Unit]
Description={0} User Service
After=network.target

[Service]
Type=simple
ExecStartPre=/usr/bin/dbus-daemon --session --fork
EnvironmentFile={1}
Environment=DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/%U/bus
WorkingDirectory=/home/{0}/build
ExecStart={2}
Restart=on-failure

[Install]
WantedBy=default.target",
        service_name, env_path, run_script_path
    );

    let mut file = File::create(service_file_path).expect("Failed to create service file");
    file.write_all(service_content.as_bytes())
        .expect("Failed to write to service file");
    println!("Service file created at: {}", service_file_path);

    // Create a symbolic link in default.target.wants
    let target_wants_dir = format!(
        "/home/{}/.config/systemd/user/default.target.wants",
        service_name
    );
    std::fs::create_dir_all(&target_wants_dir)
        .expect("Failed to create default.target.wants directory");

    let symlink_path = format!("{}/{}.service", target_wants_dir, service_name);

    // Check if the symbolic link exists and remove it if it does
    if std::path::Path::new(&symlink_path).exists() {
        std::fs::remove_file(&symlink_path).expect("Failed to remove existing symbolic link");
    }

    // Create the new symbolic link
    symlink(service_file_path, symlink_path).expect("Failed to create symbolic link");
}

/*
I hate all of this, I don't know what a good alternative to this is, I thought that systemd was a server thing
that didn't require a desktop setup for users. I'm confused as to why we need these to ensure that systemctl
works and allows the services to run.

The purpose of metal-deploy is to run on servers and possibly ubuntu-server, we shouldn't need this fakery to
pretend that this is a desktop user. Need to investigate the proper way of doing this.

The environment variables DBUS_SESSION_BUS_ADDRESS and XDG_RUNTIME_DIR are needed for managing user sessions and
communication between applications in a user session context. These variables need particular attention for each
user shell:
XDG_RUNTIME_DIR:
    This directory is intended for user-specific non-essential runtime files and other abstract resources. It's
    used by applications and desktop environments to store information that shouldn't be exposed across session
    restarts.
    Usage: It's generally set to /run/user/<user_id> where <user_id> is the numerical user ID. This directory
    facilitates file-based communication between processes within a session.
    Need for Instantiation: For users created without a complete desktop environment or display manager support
    (as with command-line script creation), this directory won’t automatically be available. Hence, it must be
    manually defined in each shell session to ensure correct application behavior.

DBUS_SESSION_BUS_ADDRESS:
    This variable points to the D-Bus session bus, a message bus application framework that allows processes to
    communicate with one another. D-Bus is widely used in Linux desktop environments to manage communication for
    various services, such as launching applications or interacting with system services.
    Usage: It often takes the form unix:path=<path_to_socket>, where <path_to_socket> is a Unix domain socket
    location provided by a D-Bus daemon instance tailored to the user session.
    Need for Instantiation: Daemon processes like dbus-daemon might not run automatically in non-graphical login
    setups unless manually initiated. This necessitates the variable assignment within each session to support
    applications dependent on D-Bus messaging.

Why Instantiation is Required with Script-Created Users:

    Lack of Automatic Environment Setup: In a default graphical session, display managers typically handle the
    creation of these variables because they initialize a fully-fledged user session. Command-line created users,
    however, lack this level of integration, requiring explicit scripting or service configuration to manually set
    these environment variables.
    Session Services: Automatic management of these variables often ties into larger infrastructure pieces such as
    the session manager, desktop environment, and systemd services — which aren't instantiated in script-created
    user contexts.

To ensure robust and correct user environment setup for script-created users, you must explicitly configure or script
the initialization of these environment variables whenever a shell session is established for the user. This includes
starting dbus-daemon if necessary and setting up XDG_RUNTIME_DIR. By doing this, you can mimic the default user setup
behavior that is normally managed by desktop environments and display managers.

These two are needed for this:
```
$ systemctl --user <command>
```

If the env variables are not exported, you get this:
```
$ systemctl --user status
Failed to connect to bus: No medium found
$
```

*/
