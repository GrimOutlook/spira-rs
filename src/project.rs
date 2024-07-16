use std::sync::Arc;
use log::trace;

use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use time::OffsetDateTime;

use crate::{client::SpiraClient, json_value, parse_date, SpiraError};
use crate::requirement::Requirement;

#[derive(Debug)]
pub struct Project {
    client: Arc<SpiraClient>,
    id: u64,
    name: String,
    description: String,
    creation_date: OffsetDateTime,
}

impl Project {
    pub async fn requirements_count(&self) -> Result<u64, SpiraError> {
        let request_string = format!("projects/{}/requirements/count" , self.id);
        let text = self.client.request(&request_string)
            .await?
            .text()
            .await?;

        #[cfg(feature = "log")]
        trace!("Requirements count response: {}", text);

        Ok(text.parse().unwrap())
    }

    pub async fn requirements(&self) -> Result<Vec<Requirement>, SpiraError> {
        // If the starting row is 0 then nothing gets returned. This is not stated in documentation
        // and confused me for an hour or two. Just leave it at 1.
        let starting_requirement = 1;
        let request_text = format!(
            "projects/{}/requirements?starting_row={}&number_of_rows={}" ,
            self.id, starting_requirement, self.requirements_count().await?
        );
        let response_text = self.client.request(&request_text)
            .await?
            .text()
            .await?;

        #[cfg(feature = "log")]
        trace!("Requirements response: {}", response_text);

        return Requirement::requirements_from_json(&response_text, &self.client);
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Takes in an array of projects in JSON form.
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
        where D: Deserializer<'de>,
    {

        let project = &Value::deserialize(deserializer)?;

        let project = Project {
            client: Arc::from(self.client.clone()),
            id: json_value("ProjectId", project).as_u64().unwrap(),
            name: json_value("Name", project).as_str().unwrap().to_string(),
            description: json_value("Description", project).as_str().unwrap().to_string(),
            creation_date: parse_date(json_value("CreationDate", project).as_str().unwrap()),
        };

        println!("{:?}", project);

        Ok(project)
    }
}