# investec-api-rust
A rust client for [Investec APIs](https://developer.investec.com/za/api-products/documentation/SA_PB_Account_Information#section/Introduction).

> Still WIP. Only oauth2 and get_accounts are implemented.

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

Once the client is created, making request to the endpoints are simple.
e.g. getting accounts:

```rust
client.get_accounts().await?;
```

## Roadmap

- [@] implement the rest of the endpoints
- [ ] full test coverage
- [ ] publish on crates.io
- [ ] wasm support
