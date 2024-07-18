extern crate spira_rs;

use std::env;

use spira_rs::client::SpiraClient;
use spira_rs::requirement::Requirement;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let base_url = env::var("BASE_SPIRA_URL").expect("The BASE_SPIRA_URL environment variable is not set");
    let version = spira_rs::SupportSpiraVersions::V5_0;
    let username = env::var("SPIRA_USERNAME").expect("The SPIRA_USERNAME environment variable is not set");
    let api_key = env::var("SPIRA_API_KEY").expect("The SPIRA_API_KEY environment variable is not set");
    let client = SpiraClient::new(&base_url, version, &username, &api_key);

    // Get all the projects we have access to and print them
    let projects = client.projects().await.unwrap();
    projects.iter().for_each(|p| println!("Found project: {}", p.name()));

    // Get a specific project with a known ID.
    let project = projects.get(0).unwrap();
    println!("Project ID [{}] is named [{}] and has [{}] requirements", project.id(), project.name(), project.requirements_count().await.unwrap());

    // Get the requirements from this project.
    let req = project.requirements().await.unwrap();
    req.iter().for_each(|r| println!("Found requirement: {}", r.name()));

    // Get the first requirement for the rest if this example
    let first_req = req.get(0).unwrap();
    println!("Requirement name: {}. Custom Prop: {:?}", first_req.name(), first_req.custom_properties());
}