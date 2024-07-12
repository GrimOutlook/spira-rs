use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use crate::project::Project;

pub mod client;
pub mod project;
mod client_container;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SupportSpiraVersions {
    V7_0,
    V6_0,
    V5_0,
    V4_0,
}


#[derive(Debug)]
pub enum SpiraError {
    SerdeJSONParsingError(serde_json::Error),
    JSONParsingError(String),
    RequestError(reqwest::Error)
}

impl From<serde_json::Error> for SpiraError {
    fn from(value: serde_json::Error) -> Self {
        return SpiraError::SerdeJSONParsingError(value)
    }
}

impl From<reqwest::Error> for SpiraError {
    fn from(value: reqwest::Error) -> Self {
        return SpiraError::RequestError(value)
    }
}

pub(crate) fn json_value(key: &str, project: &serde_json::Value) -> serde_json::Value {
    project.get(key).expect(&format!("No {} found in project", key)).clone()
}

pub(crate) fn parse_date(date_str: &str) -> DateTime<FixedOffset> {
    return DateTime::parse_from_str(date_str, "%s%3f%z")
        .expect(&format!("Cannot parse date from string [{}]", date_str));
}