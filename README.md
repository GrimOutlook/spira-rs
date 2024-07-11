# spira-rs
SpiraTest API bindings for Rust 🦀

This is a fork of [spira by alxolr](https://github.com/alxolr/spira).
At the time I forked this repo, it hadn't been touched in 2 years and was missing almost all of features that I needed but their code is the basis for this codebase.

Bindings are currently tested for SpiraTest v5.4.0.4 which is the version I use at work.
I developed these in my free time, so I could let other people use them.
I want to eventually support present day SpiraTest but that is a less pressing matter as we currently don't utilize it.

Contributions are welcome 🙂

## Usage

```rust
use spira::SpiraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_API_URL")?;

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    let projects = spira_client.projects().await?;

    println!("{:#?}", projects);
    Ok(())
}
```

## Documentation

Crate [spira-rs@0.1.0](https://docs.rs/spira-rs/0.1.0/) docs

This application only supports REST API calls.

## Test

### Test Command

`RUST_LOG=trace BASE_SPIRA_URL="https://demo-us.spiraservice.net/aimai" SPIRA_USERNAME="administrator" SPIRA_API_KEY="" cargo run --features log --example list_projects`

---

## Reference Material

- [SpiraPlan: REST Web Service (v5.0): API Documentation](https://api.inflectra.com/spira/services/v5_0/RestService.aspx)
- [SpiraPlan: SoapService SOAP Web Service (v5.0): API Documentation](https://api.inflectra.com/Spira/Services/v5_0/SoapService.aspx)