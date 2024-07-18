use std::sync::Arc;

use reqwest::{header::{HeaderMap, HeaderValue}, ClientBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{project::Project, SpiraError, SupportSpiraVersions};

#[cfg(feature = "log")]
use log::trace;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpiraClient {
    base_url: Arc<str>,
    username: Arc<str>,
    api_key: Arc<str>,
    version: SupportSpiraVersions,
}

impl SpiraClient {
    pub fn new(base_url: &str, version: SupportSpiraVersions, username: &str, api_key: &str) -> Arc<SpiraClient> {
        let client = SpiraClient {
            base_url: base_url.into(),
            username: username.into(),
            api_key: api_key.into(),
            version,
        };

        return Arc::new(client);
    }

    pub async fn projects(&self) -> Result<Vec<Project>, SpiraError> {
        #[cfg(feature = "log")]
        trace!("Getting projects accessible by user {}", self.username);

        let text = self.request("projects")
            .await?
            .text()
            .await?;

        #[cfg(feature = "log")]
        trace!("Projects response: {}", text);

        return Project::projects_from_json(&text, self);
    }

    pub async fn project_by_id(&self, project_id: u64) -> Result<Option<Project>, SpiraError> {
        let request_text = format!("projects/{}", project_id);
        let response_text = self.request(&request_text)
            .await?
            .text()
            .await?;

        #[cfg(feature = "log")]
        trace!("Project response: {}", response_text);

        if response_text == "\"Cannot find the supplied project id in the system\"" {
            return Ok(None)
        }

        let project = Project::project_from_json(&response_text, &self)?;
        return Ok(Some(project))
    }

    pub async fn project_by_name(&self, project_name: &str) -> Result<Option<Project>, SpiraError> {
        // There is no way to query the API for a project by its name, so we need to just gather
        // all of them and search for the project by name ourselves.
        let projects = self.projects().await?;
        Ok(projects.into_iter().find(|r| r.name().eq(project_name.into())))
    }

    pub(crate) async fn request(&self, req: &str) -> reqwest::Result<Response> {
        let mut headers = HeaderMap::new();
        
        headers.insert("username", HeaderValue::from_str(&self.username).expect("Username value is invalid"));
        headers.insert("api-key", HeaderValue::from_str(&self.api_key).expect("API Key is invalid"));
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert("Content-type", HeaderValue::from_str("application/json").unwrap());
        
        let client = ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build()?;

        let url = format!("{}/Services/{}/RestService.svc/{}", self.base_url, self.url_version(), req);
        
        #[cfg(feature = "log")]
        trace!("Getting response from: {}", url);

        return client.get(url).send().await;
    }

    fn url_version(&self) -> &str {
        match self.version {
            SupportSpiraVersions::V5_0 => "v5_0",
            v => todo!("Version [{:?}] is not currently supported", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::{json, Value};
    use crate::client::SpiraClient;
    use crate::SupportSpiraVersions;

    static BASE: &str = "/Services/v5_0/RestService.svc";

    fn get_json_project(name: &str, id: u64) -> Value {
        json!({
            "Active": true,
            "CreationDate":"/Date(1707863960317-0600)/",
            "Description":"<p>This is a description.</p><p>&nbsp;</p><p>Second line of description</p>",
            "Name":name,
            "NonWorkingHours":0,
            "ProjectId": id,
            "Website":"",
            "WorkingDays":5,
            "WorkingHours":8
        })
    }

    #[tokio::test]
    async fn projects() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/projects", BASE));
            then.status(200)
                .body(Value::to_string(
                    &Value::Array(
                        vec![
                            get_json_project("Project1", 5),
                            get_json_project("Project2", 6)
                            ]
                        )));
        });
        let url = server.url("");

        let client = SpiraClient::new(&*url, SupportSpiraVersions::V5_0, "", "");
        let projects = client.projects().await.unwrap();
        assert_eq!(projects.len(), 2);
        let project = projects.get(0).unwrap();
        assert_eq!(project.name(), "Project1");
    }

    #[tokio::test]
    async fn project_by_id() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/projects/5", BASE));
            then.status(200)
                .body(Value::to_string(&get_json_project("Project1", 5)));
        });
        let url = server.url("");

        let client = SpiraClient::new(&*url, SupportSpiraVersions::V5_0, "", "");
        let project = client.project_by_id(5).await.unwrap().unwrap();
        assert_eq!(project.name(), "Project1");
    }

    #[tokio::test]
    async fn project_by_name() {
        let server = MockServer::start();

        server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}/projects", BASE));
            then.status(200)
                .body(Value::to_string(
                    &Value::Array(
                        vec![
                            get_json_project("Project1", 5),
                            get_json_project("Project2", 6)
                            ]
                        )));
        });
        let url = server.url("");

        let client = SpiraClient::new(&*url, SupportSpiraVersions::V5_0, "", "");
        let project = client.project_by_name("Project2").await.unwrap().unwrap();
        assert_eq!(project.name(), "Project2");
        assert_eq!(*project.id(), 6);
    }
}
