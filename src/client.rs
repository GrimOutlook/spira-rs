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
    use super::*;

    #[test]
    fn it_works() {
    }
}
