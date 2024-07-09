use serde::{Deserialize, Deserializer, Serialize};

use crate::client::SpiraClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub(crate) client: SpiraClient,

    #[serde(rename = "ProjectId")]
    pub(crate) id: u64,
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "Description")]
    pub(crate) description: String,
    #[serde(rename = "CreationDate")]
    pub(crate) creation_date: String,
}

impl Project {
    pub fn id(&self) -> u64 {
        self.id
    }
}