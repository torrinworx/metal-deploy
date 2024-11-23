use std::io::{self, Write};

pub fn confirm(prompt: &str) -> bool {
	print!("{}", prompt);
	print!(" [Y/n]: ");
	io::stdout().flush().unwrap();

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	let input = input.trim();
	input.is_empty() || input.to_lowercase() == "y"
}
