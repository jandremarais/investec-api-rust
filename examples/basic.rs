use std::collections::HashSet;

use chrono::NaiveDate;
use investec::client::{Client, ClientBuilder, TransactionType};

#[tokio::main]
async fn main() -> Result<(), investec::Error> {
    // load credentials from env
    let mut client = ClientBuilder::from_env()
        // save a copy of the token locally as a cache
        .local_token()
        // automatically refresh tokens if expired or non-existent
        .refresh_auth()
        .build()?;

    // for a client pointing to the sandbox environment
    // let mut client = Client::sandbox();

    // uncomment below if not using .refresh_auto()
    // client.authenticate().await?;

    let accounts = client.get_accounts().await?;
    println!("Accounts:\n");
    let from_date = NaiveDate::from_ymd_opt(2023, 10, 1).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2023, 10, 3).unwrap();
    let t_type = TransactionType::CardPurchases;
    for a in accounts.data.accounts.iter().take(1) {
        println!("{}", a);
        println!("---");
        println!("Balance:");
        let balance = client.get_account_balnce(&a.account_id).await?;
        println!("{}\n", balance.data);

        let transactions = client
            .get_account_transactions(&a.account_id, Some(from_date), Some(to_date), Some(t_type))
            .await?;

        dbg!(transactions);
    }

    Ok(())
}
