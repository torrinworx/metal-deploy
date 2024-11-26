use crate::utils::systemctl::systemctl;
use std::path::Path;

/*
Update a service with the latest commits on github, or the latest release.
*/
pub fn run(service_name: String) {
    println!("Updating service: {}", service_name);
}
