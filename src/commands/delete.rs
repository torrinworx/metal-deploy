/*
Delete a service, if a service is still running ask the user if they're sure they want to delete a running service?
Delete the user folder and all service/config/env files.

Before deleting the service ask them if they would like to delete all data, all env variables, and all service data/content.
*/

pub fn run(service_name: String) {
	println!("Deleting service: {}", service_name);
}
