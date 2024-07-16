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

    let projects = client.projects().await.unwrap();
    for p in projects {
        println!("Project ID [{}] is named [{}] and has [{}] requirements", p.id(), p.name(), p.requirements_count().await.unwrap());
        let req = p.requirements().await.unwrap();
        let first_req = req.get(0).unwrap();
        println!("Requirement name: {}. Custom Prop: {:?}", first_req.name(), first_req.custom_properties());
    }
}