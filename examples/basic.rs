use investec::client::{Client, ClientBuilder};

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
    for a in accounts.data.accounts.iter() {
        println!("{}", a);
        println!("---");
        println!("Balance:");
        let balance = client.get_account_balnce(&a.account_id).await?;
        println!("{}\n", balance.data);
    }

    Ok(())
}
