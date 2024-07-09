use serde::{Deserialize, Serialize};

pub mod client;
pub mod project;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SupportSpiraVersions {
    V7_0,
    V6_0,
    V5_0,
    V4_0,
}