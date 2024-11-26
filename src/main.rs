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
        #[arg(short, long)]
        branch: Option<String>,
    },
    Build {
        service_name: String,
        #[arg(short, long, default_value_t = true)]
        replace_existing: bool,
    },
    Start {
        service_name: String,
        #[arg(short, long, default_value_t = false)]
        skip_env_creation: bool,
    },
    Stop {
        service_name: String,
    },
    Delete {
        service_name: String,
    },
    Restart {
        service_name: String,
    },
    Update {
        service_name: String,
        #[arg(long, default_value_t = false)]
        latest_release: bool,
    },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            repo_url,
            name,
            branch,
        } => commands::add::run(repo_url, name, branch),
        Commands::Build {
            service_name,
            replace_existing,
        } => commands::build::run(service_name, replace_existing),
        Commands::Delete { service_name } => commands::delete::run(service_name),
        Commands::List => commands::list::run(),
        Commands::Start {
            service_name,
            skip_env_creation,
        } => commands::start::run(service_name, skip_env_creation),
        Commands::Stop { service_name } => commands::stop::run(service_name),
        Commands::Restart { service_name } => commands::restart::run(service_name),
        Commands::Update {
            service_name,
            latest_release,
        } => commands::update::run(service_name, latest_release),
    }
}
