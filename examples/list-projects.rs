use std::env;
use spira_rs::SpiraClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the API key, username, and base URL from environment variables.
    let api_key = match env::var("SPIRA_API_KEY") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("SPIRA_API_KEY environment variable not set");
            return Err(e.into());
        }
    };
    let username = match env::var("SPIRA_USERNAME") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("SPIRA_USERNAME environment variable not set");
            return Err(e.into());
        }
    };
    let base_url = match env::var("SPIRA_API_URL") {
        Ok(key) => key,
        Err(e) => {
            eprintln!("SPIRA_API_URL environment variable not set");
            return Err(e.into());
        }
    };

    let spira_client = SpiraClient::new(&base_url, &api_key, &username)?;
    let projects = spira_client.project.list().await?;
    let project_names = projects.into_iter().map(|project| project.name.unwrap())
        .collect::<Vec<String>>();

    println!("{:?}", project_names);
    Ok(())
}