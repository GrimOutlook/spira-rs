use serde::{Deserialize, Serialize};

pub mod client;
pub mod project;
mod client_loader;
mod playground;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SupportSpiraVersions {
    V7_0,
    V6_0,
    V5_0,
    V4_0,
}


#[derive(Debug)]
pub enum SpiraError {
    JSONParsingError(String),
    RequestError(reqwest::Error)
}

impl From<reqwest::Error> for SpiraError {
    fn from(value: reqwest::Error) -> Self {
        return SpiraError::RequestError(value)
    }
}