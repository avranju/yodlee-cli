use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;
use yodlee_rs::Client;

use crate::UserCommand;

pub async fn process_command(client: &mut Client, command: UserCommand) -> Result<()> {
    match command {
        UserCommand::Register { json_file } => {
            register_user(client, json_file).await?;
        }
        UserCommand::Get { login_name } => {
            get_user(client, &login_name).await?;
        }
        UserCommand::Delete { login_name } => {
            delete_user(client, &login_name).await?;
        }
    }

    Ok(())
}

async fn get_user(client: &mut Client, login_name: &str) -> Result<()> {
    let mut user = client.user(login_name.to_string()).await?;
    let res = user.get_details().await?;

    println!("{}", serde_json::to_string_pretty(&res)?);

    Ok(())
}

async fn delete_user(client: &mut Client, login_name: &str) -> Result<()> {
    let mut user = client.user(login_name.to_string()).await?;
    let _ = user.delete().await?;

    println!("User '{login_name}' deleted.");

    Ok(())
}

async fn register_user(client: &mut Client, json: PathBuf) -> Result<()> {
    let json = fs::read_to_string(json).await?;
    let req = serde_json::from_str(&json)?;
    let res = client.register_user(req).await?;

    println!("{}", serde_json::to_string_pretty(&res)?);

    Ok(())
}
