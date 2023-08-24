use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set(SetArgs),
    /// Get the string value of a given string key
    Get(GetArgs),
    /// Remove a given key
    Rm(RmArgs),
}

#[derive(Args)]
struct SetArgs {
    key: String,
    value: String,
}

#[derive(Args)]
struct GetArgs {
    key: String,
}

#[derive(Args)]
struct RmArgs {
    key: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Set(_) => {
            panic!("unimplemented");
        }
        Commands::Get(_) => {
            panic!("unimplemented");
        }
        Commands::Rm(_) => {
            panic!("unimplemented");
        }
    }
}
