# investec-api-rust
A rust client for [Investec APIs](https://developer.investec.com/za/api-products/documentation/SA_PB_Account_Information#section/Introduction).

> WIP. See Roadmap below.

The goal of this crate is to make it easy to call Investec APIs from Rust programs.

## Usage

Create the client by reading the client_id, client_secret and api_key from environment variables and authenitcate to get an access token:

```sh
# .env
INVESTEC_CLIENT_ID=
INVESTEC_CLIENT_SECRET=
INVESTEC_API_KEY=
```

```rust
let mut client = ClientBuilder::from_env().build()?;
client.authenticate()?;
````

To enable auto refresh of the access tokens and caching to local file system, create the client as follows:

```rust
let client = ClientBuilder::from_env()
    .local_token()
    .refresh_auth()
    .build()?;
```

If you want the client to point to the sandbox environment:

```rust
let client = Client::sandbox();
```

Once the client is created, making requests to the endpoints are simple.
e.g. getting accounts:

```rust
let accounts = client.get_accounts().await?;
```

or getting account transactions:

```rust
let from_date = NaiveDate::from_ymd_opt(2023, 10, 1);
let to_date = NaiveDate::from_ymd_opt(2023, 10, 3);
let t_type = TransactionType::CardPurchases;
let transactions = client
    .get_account_transactions("1234", from_date, to_date, Some(t_type))
    .await?;
```

See [examples/basic.rs](examples/basic.rs) for an end-to-end example.
You can run it with:
```sh
cargo run --examples basic
```

## Roadmap

- [x] implement account info endpoints
- [x] implement inter account transfers
- [x] add basic example
- [x] basic documentation
- [ ] implement beneficiary payments endpoints
- [ ] implement document endpoints
- [ ] add example for account transfer
- [ ] add example for beneficary payments
- [ ] better error management and test coverage for errors
- [ ] publish on crates.io
- [ ] wasm support
