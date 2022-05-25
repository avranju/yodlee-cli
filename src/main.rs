use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use yodlee_rs::Client;

mod account;
mod user;

/// Yodlee command line app
#[derive(Parser, Debug)]
#[clap(version)]
struct Opts {
    /// Yodlee API endpoint
    #[clap(short = 'e', long, env = "YODLEE_API_ENDPOINT")]
    api_endpoint: String,

    /// Yodlee API version
    #[clap(short = 'v', long, env = "YODLEE_API_VERSION")]
    api_version: String,

    /// Admin login name
    #[clap(short = 'a', long, env = "YODLEE_ADMIN_LOGIN_NAME")]
    admin_login_name: String,

    /// Yodlee client ID
    #[clap(short = 'c', long, env = "YODLEE_CLIENT_ID")]
    client_id: String,

    /// Yodlee client secret
    #[clap(short = 's', long, env = "YODLEE_CLIENT_SECRET")]
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

    /// Account management commands
    #[clap(subcommand)]
    Account(AccountCommand),
}

#[derive(Subcommand, Debug)]
pub enum AccountCommand {
    /// List accounts held by user
    Get {
        /// The user's login name
        #[clap(short, long)]
        login_name: String,
    },

    /// Delete an account
    Delete {
        /// The user's login name
        #[clap(short, long)]
        login_name: String,

        /// The ID of the account to be deleted
        #[clap(short, long)]
        account_id: String,
    },

    /// List history of account balances
    History {
        /// The user's login name
        #[clap(short = 'l', long)]
        login_name: String,

        /// Consider carry forward logic for missing balances
        #[clap(short = 'c', long)]
        include_carry_forward: Option<bool>,

        /// Date from which balances should be retrieved (YYYY-MM-DD)
        #[clap(short = 'f', long)]
        from_date: Option<String>,

        /// Date till which balances should be retrieved (YYYY-MM-DD)
        #[clap(short = 't', long)]
        to_date: Option<String>,

        /// d-daily, w-weekly or m -monthly
        #[clap(short = 'i', long)]
        interval: Option<String>,

        /// UNRECONCILED (default), or RECONCILED
        #[clap(short = 'r', long)]
        account_reconcile_type: Option<String>,

        /// Skip the first n records (min: 0)
        #[clap(short = 's', long)]
        skip: Option<u32>,

        /// Fetch top N records (max: 500)
        #[clap(short = 'o', long)]
        top: Option<u32>,

        /// Account for which the history should be retrieved
        #[clap(short = 'a', long)]
        account_id: Option<String>,
    },
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
        Command::Account(command) => account::process_command(&mut client, command).await?,
    }

    Ok(())
}
