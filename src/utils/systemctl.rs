use std::path::Path;
use std::process::Command;

pub fn systemctl(service_name: &str, command: &str) {
    let user_id = Command::new("id")
        .args(&["-u", service_name])
        .output()
        .expect("Failed to get user id")
        .stdout;
    let user_id_str = String::from_utf8_lossy(&user_id).trim().to_string();

    let xdg_runtime_dir = format!("/run/user/{}", user_id_str);
    let dbus_address = format!("unix:path={}/bus", xdg_runtime_dir);

    // Check and start `dbus-daemon` for the user if necessary
    let bus_path_string = format!("/run/user/{}/bus", user_id_str);
    let bus_path = Path::new(&bus_path_string);
    if !bus_path.exists() {
        let status = Command::new("sudo")
            .arg("-u")
            .arg(service_name)
            .arg("dbus-daemon")
            .arg("--session")
            .arg("--fork")
            .arg("--address")
            .arg(&dbus_address)
            .status();

        if let Err(e) = status {
            eprintln!(
                "Failed to start dbus-daemon for user {}: {}",
                service_name, e
            );
            return;
        }
    }

    if !bus_path.exists() {
        eprintln!("D-Bus session bus not available for user {}", service_name);
        return;
    }

    // Run the systemctl command
    let mut env_command = Command::new("su");
    env_command
        .arg("-l")
        .arg(service_name)
        .arg("-c")
        .arg(format!(
			"export DBUS_SESSION_BUS_ADDRESS={} && export XDG_RUNTIME_DIR={} && systemctl --user {} {}",
			dbus_address, xdg_runtime_dir, command, service_name
		))
        .env("DBUS_SESSION_BUS_ADDRESS", &dbus_address);

    if let Err(e) = env_command.status() {
        eprintln!("Failed to run systemctl command '{}': {}", command, e);
    }
}
