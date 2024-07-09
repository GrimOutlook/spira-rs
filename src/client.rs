use std::sync::Arc;

use reqwest::{header::{HeaderMap, HeaderValue}, ClientBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{project::Project, SupportSpiraVersions};

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
    pub fn new(base_url: &str, version: SupportSpiraVersions, username: &str, api_key: &str) -> SpiraClient {
        SpiraClient {
            base_url: base_url.into(),
            username: username.into(),
            api_key: api_key.into(),
            version,
        }
    }

    pub async fn projects(&self) -> reqwest::Result<Vec<Project>> {
        #[cfg(feature = "log")]
        trace!("Getting projects accessible by user {}", self.username);

        let response = self.request("projects").await?;
        let text = response.text().await?;

        #[cfg(feature = "log")]
        trace!("Projects response: {}", text);

        let project: Vec<Project> = serde_json::from_str(&text).expect("Error getting response text");

        Ok(project)
    }

    async fn request(&self, req: &str) -> reqwest::Result<Response> {
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

impl Default for SpiraClient {
    fn default() -> Self {
        Self {
            base_url: "".into(),
            username: "".into(),
            api_key: "".into(),
            version: SupportSpiraVersions::V7_0
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
