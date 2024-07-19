use std::sync::Arc;
use derive_getters::Getters;
#[cfg(feature = "log")]
use log::trace;

use serde::{de::DeserializeSeed, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use time::OffsetDateTime;

use crate::{client::SpiraClient, json_value, parse_date, SpiraError};
use crate::requirement::Requirement;

#[derive(Debug, Getters)]
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

        Ok(text.parse().expect(&format!("Can't parse requirement count: [{}]", text)))
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

    pub async fn requirement_by_id(&self, requirement_id: u64) -> Result<Option<Requirement>, SpiraError> {
        let request_text = format!("projects/{}/requirements/{}", self.id, requirement_id);
        let response_text = self.client.request(&request_text)
            .await?
            .text()
            .await?;

        #[cfg(feature = "log")]
        trace!("Requirement response: {}", response_text);

        if response_text == "\"Cannot find the supplied requirement id in the system\"" {
            return Ok(None)
        }

        let req = Requirement::requirement_from_json(&response_text, &self.client)?;
        return Ok(Some(req))
    }

    pub async fn requirement_by_name(&self, requirement_name: &str) -> Result<Option<Requirement>, SpiraError> {
        // There is no way to query the API for a requirement by its name, so we need to just gather
        // all of them and search for the requirement by name ourselves.
        let requirements = self.requirements().await?;
        Ok(requirements.into_iter().find(|r| r.name().eq(requirement_name)))
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

impl Default for Project {
    fn default() -> Self {
        Self {
            client: Default::default(),
            id: Default::default(),
            name: Default::default(),
            description: Default::default(),
            creation_date: OffsetDateTime::now_utc()
        }
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

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::Value;
    use crate::client::SpiraClient;
    use crate::SupportSpiraVersions;

    use super::Project;

    static BASE: &str = "/Services/v5_0/RestService.svc";

    fn get_project(url: String) -> Project {
        let client = SpiraClient::new(&*url, SupportSpiraVersions::V5_0, "", "");
        Project {
            client,
            id: 5,
            ..Default::default()
        }
    }

    fn get_json_requirement(name: &str, id: u64) -> Value {
        // Get the requirement JSON from an external file
        let mut value = serde_json::from_str(include_str!("test_jsons/requirement.json")).unwrap();
        
        // We need to be able to modify the JSON for testing purposes. Change the requirement name and ID here.
        if let Value::Object(v) = &mut value {
            v.extend([
                ("Name".to_owned(), name.into()),
                ("RequirementId".to_owned(), id.into()),
            ])
        }

        return value;
    }

    #[tokio::test]
    async fn requirements() {
        let server = MockServer::start();

        let requirements_request = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/projects/5/requirements", BASE))
                .query_param("starting_row", "1")
                .query_param("number_of_rows", "2");
            then.status(200)
                .body(Value::to_string(
                    &Value::Array(
                        vec![
                            get_json_requirement("Requirement 1", 5),
                            get_json_requirement("Requirement 2", 6)
                        ]
                    )));
        });

        // When getting the requirements it has to get the total number first.
        let count_request = server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/projects/5/requirements/count", BASE));
            then.status(200)
                .body("2");
        });
        let url = server.url("");
        let requirements = get_project(url).requirements().await.unwrap();
        
        count_request.assert();
        requirements_request.assert();
        assert_eq!(requirements.len(), 2);
        
        let requirement = requirements.get(0).unwrap();
        assert_eq!(requirement.name(), "Requirement 1");
    }
}
