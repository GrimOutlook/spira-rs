use std::sync::Arc;

use serde::{de::DeserializeSeed, Deserialize, Serialize};

use crate::{client::SpiraClient, client_loader::ClientLoader, SpiraError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub(crate) client: Arc<SpiraClient>,

    #[serde(rename = "ProjectId")]
    pub(crate) id: u64,
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "Description")]
    pub(crate) description: String,
    #[serde(rename = "CreationDate")]
    pub(crate) creation_date: String,
}

impl Project {
    pub async fn requirements(&self) {
        let a = format!("projects/{}/requirements" , self.id);
        let requirements = self.client.request(&a).await;
        println!("Reqs: {:?}", requirements.unwrap())
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Takes in an array of projects in JSON form.
    pub(crate) fn projects_from_json(json_string: &str, client: &SpiraClient) -> Result<Vec<Project>, SpiraError> {
        let Ok(projects_json): Result<serde_json::Value,_> = serde_json::from_str(json_string) else {
            return Err(SpiraError::JSONParsingError(format!("Projects JSON is invalid: {}", json_string)))
        };

        let Some(projects_array) = projects_json.as_array() else {
            return Err(SpiraError::JSONParsingError(format!("Projects JSON is not an array: {}", json_string)))
        };

        let mut project_vec = Vec::new();

        for project_json in projects_array {
            let project = Project::project_from_json(&project_json.to_string(), client)?;
            project_vec.push(project);
        }

        return Ok(project_vec);
    }

    // Implement this https://stackoverflow.com/questions/63306229/how-to-pass-options-to-rusts-serde-that-can-be-accessed-in-deserializedeseria
    pub(crate) fn project_from_json(json_string: &str, client: &SpiraClient) -> Result<Project, SpiraError> {
        todo!();
        let mut deserializer = serde_json::Deserializer::new(serde_json::de::StrRead::new(json_string));
        // ProjectDeserializer { client }
    }
}