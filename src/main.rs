use clap::{Parser, Subcommand};
use regex::Regex;
use std::process::Command;

#[derive(Parser)]
#[command(author = "Torrin Leonard", version = "1.0", about = "A tool for managing and deploying git-based applications", long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Add {
		repo_url: String,
		#[arg(short, long)]
		name: Option<String>,
	},
	Build { service_name: String },
	Start { service_name: String },
	Stop { service_name: String },
	Delete { service_name: String },
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Add { repo_url, name } => add_service(repo_url, name),
		Commands::Build { service_name } => build_service(service_name),
		Commands::Start { service_name } => start_service(service_name),
		Commands::Stop { service_name } => stop_service(service_name),
		Commands::Delete { service_name } => delete_service(service_name),
	}
}

fn add_service(repo_url: String, name: Option<String>) {
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

	// Command to add a new user
	let output = Command::new("sudo")
		.arg("useradd")
		.arg("-m")  // Create a home directory
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

fn build_service(service_name: String) {
	println!("Building service: {}", service_name);
}

fn start_service(service_name: String) {
	println!("Starting service: {}", service_name);
}

fn stop_service(service_name: String) {
	println!("Stopping service: {}", service_name);
}

fn delete_service(service_name: String) {
	println!("Deleting service: {}", service_name);
}
