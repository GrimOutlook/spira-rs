# spira-rs
SpiraTest API bindings for Rust 🦀

Bindings for SpiraTest v5.4.0.4 which is the version I use at work.
I developed these in my free time, so I could let other people use them.
I want to eventually support present day SpiraTest but that is a less pressing matter as we currently don't utilize it.

Contributions are welcome 🙂

## Usage

```rust
use spira::{resources::project::ProjectDto, SpiraClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("SPIRA_API_KEY")?;
    let username = env::var("SPIRA_USERNAME")?;
    let base_url = env::var("SPIRA_API_URL")?;

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    let projects = spira_client.project.list().await?;

    println!("{:#?}", projects);
    Ok(())
}
```

## Documentation

Crate [spira@0.0.6](https://docs.rs/spira/0.0.6/spira/) docs

## Task

Getting a task by id

```rust
/// ...
let task: TaskDto = spira_client.task.get(100 /* project_id */, task_id /* task_id */).await?;
```

## Requirement

Getting a requirement by id

```rust
/// ...
let requirement: RequirementDto = spira_client.requirement.get(100 /* project_id */, 1500 /* requirement_id */).await?;
```

---

## Reference Material

- [SpiraPlan: REST Web Service (v5.0): API Documentation](https://api.inflectra.com/spira/services/v5_0/RestService.aspx)
- [SpiraPlan: SoapService SOAP Web Service (v5.0): API Documentation](https://api.inflectra.com/Spira/Services/v5_0/SoapService.aspx)
