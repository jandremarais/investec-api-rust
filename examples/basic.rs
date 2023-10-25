use investec::client::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), investec::Error> {
    let mut client = ClientBuilder::from_env().local_token().build()?;
    client.authenticate().await?;
    Ok(())
}
