use anyhow::Result;
use yodlee_rs::{models::Account, Client};

use crate::AccountCommand;

pub async fn process_command(client: &mut Client, command: AccountCommand) -> Result<()> {
    match command {
        AccountCommand::Get { login_name } => {
            let accounts = get_accounts(client, &login_name).await?;
            println!("{:#?}", accounts);
        }
    }

    Ok(())
}

async fn get_accounts(client: &mut Client, login_name: &str) -> Result<Option<Vec<Account>>> {
    let mut account = client.account(login_name.to_string());
    let res = account
        .get_accounts(None, None, None, None, None, None)
        .await?;

    Ok(res.account)
}
