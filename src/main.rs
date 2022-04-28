use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use yodlee_rs::Client;

mod user;

/// Yodlee command line app
#[derive(Parser, Debug)]
#[clap(version)]
struct Opts {
    /// Yodlee API endpoint
    #[clap(short = 'e', long)]
    api_endpoint: String,

    /// Yodlee API version
    #[clap(short = 'v', long)]
    api_version: String,

    /// Admin login name
    #[clap(short = 'a', long)]
    admin_login_name: String,

    /// Yodlee client ID
    #[clap(short = 'c', long)]
    client_id: String,

    /// Yodlee client secret
    #[clap(short = 's', long)]
    client_secret: String,

    /// The command to run
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// User management commands
    #[clap(subcommand)]
    User(UserCommand),
}

#[derive(Subcommand, Debug)]
pub enum UserCommand {
    /// Register a new Yodlee user
    Register {
        /// The user's JSON configuration file. Only login name is required.
        #[clap(short, long, parse(from_os_str))]
        json_file: PathBuf,
    },

    /// Get user details
    Get {
        /// The user's login name
        #[clap(short, long)]
        login_name: String,
    },

    /// Delete user
    Delete {
        /// The user's login name
        #[clap(short, long)]
        login_name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Opts::parse();

    let mut client = Client::new(
        args.api_endpoint,
        args.api_version,
        args.admin_login_name,
        args.client_id,
        args.client_secret,
    );
    client.open().await?;

    match args.command {
        Command::User(command) => user::process_command(&mut client, command).await?,
    }

    Ok(())
}