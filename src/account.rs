use anyhow::Result;
use yodlee_rs::{account::AccountHistoricalBalanceParams, Client};

use crate::AccountCommand;

pub async fn process_command(client: &mut Client, command: AccountCommand) -> Result<()> {
    match command {
        AccountCommand::Get { login_name } => {
            let mut account = client.account(login_name);
            let accounts = account.get_accounts(Default::default()).await?;
            println!("{}", serde_json::to_string_pretty(&accounts.account)?);
        }

        AccountCommand::Delete {
            login_name,
            account_id,
        } => {
            let mut account = client.account(login_name);
            account.delete(account_id.clone()).await?;
            println!("Account {} deleted.", account_id);
        }

        AccountCommand::History {
            login_name,
            include_carry_forward,
            from_date,
            to_date,
            interval,
            account_reconcile_type,
            skip,
            top,
            account_id,
        } => {
            let mut account = client.account(login_name);
            let params = AccountHistoricalBalanceParams {
                include_carry_forward,
                from_date: from_date.as_deref(),
                to_date: to_date.as_deref(),
                interval: interval.as_deref(),
                account_reconcile_type: account_reconcile_type.as_deref(),
                skip,
                top,
                account_id: account_id.as_deref(),
            };
            let accounts = account.get_historical_balances(params).await?;
            println!("{}", serde_json::to_string_pretty(&accounts.account)?);
        }
    }

    Ok(())
}
