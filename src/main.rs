use clap::{Parser, Subcommand};
mod commands;
mod utils;

#[derive(Parser)]
#[command(
    author = "Torrin Leonard",
    version = "1.0",
    about = "A tool for managing and deploying git-based applications",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/*
TODO: find a way to auto load these commands and make this enum/match in main not needed so that each command
is modular and self contained so that there isn't so much setup for a new command.
*/
#[derive(Subcommand)]
enum Commands {
    Add {
        repo_url: String,
        #[arg(short, long)]
        name: Option<String>,
    },
    Build {
        service_name: String,
    },
    Start {
        service_name: String,
    },
    Stop {
        service_name: String,
    },
    Delete {
        service_name: String,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { repo_url, name } => commands::add::run(repo_url, name),
        Commands::Build { service_name } => commands::build::run(service_name),
        Commands::Delete { service_name } => commands::delete::run(service_name),
        Commands::List => commands::list::run(),
        Commands::Start { service_name } => commands::start::run(service_name),
        Commands::Stop { service_name } => commands::stop::run(service_name),
    }
}
