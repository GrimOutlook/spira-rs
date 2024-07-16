use regex::Regex;
use time::{OffsetDateTime};
use time_macros::format_description;
use serde::{Deserialize, Serialize};

pub mod client;
pub mod project;
pub mod requirement;

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

pub(crate) fn parse_date(raw_date_str: &str) -> OffsetDateTime {
    let re = Regex::new(r"/Date\((\d+[+-]\d+)\)/").unwrap();
    let date_str = re.captures(raw_date_str)
            .expect(&format!("Cannot parse date string [{}]", raw_date_str))
            .get(1)
            .expect(&format!("Could not find a valid timestamp in [{}]", raw_date_str))
            .as_str();
    let format = format_description!(
        "[unix_timestamp precision:millisecond][offset_hour][offset_minute]"
    );
    return OffsetDateTime::parse(date_str, &format)
        .expect(&format!("Cannot parse date from string [{}]", date_str));
}