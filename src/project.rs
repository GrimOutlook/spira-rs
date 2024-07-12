use std::sync::Arc;

use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use chrono::{DateTime, FixedOffset};

use crate::{client::SpiraClient, json_value, parse_date, SpiraError};
use crate::client_container::ClientContainer;

#[derive(Debug)]
pub struct Project {
    pub(crate) client: Arc<SpiraClient>,
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) creation_date: DateTime<FixedOffset>,
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

        // let projects_json: serde_json::Value = serde_json::from_str(json_string)?;

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
        let mut deserializer = serde_json::Deserializer::new(serde_json::de::StrRead::new(json_string));
        Ok(ProjectDeserializer { client }.deserialize(&mut deserializer)?)
    }
}

struct ProjectDeserializer<'a> {
    client: &'a SpiraClient,
}

impl<'de> DeserializeSeed<'de> for ProjectDeserializer<'_>
{
    type Value = Project;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
    {

        let project = &Value::deserialize(deserializer)?;

        let project = Project {
            client: Arc::from(self.client.clone()),
            id: json_value("ProjectId", project).as_u64().unwrap(),
            name: json_value("Name", project).as_str().unwrap().to_string(),
            description: json_value("Description", project).as_str().unwrap().to_string(),
            creation_date: parse_date(json_value("CreationDate", project).as_str().unwrap()),
        };

        Ok(project)
    }
}