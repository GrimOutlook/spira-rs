use std::sync::Arc;
use derive_getters::Getters;
use serde::de::DeserializeSeed;
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use strum::FromRepr;
use time::OffsetDateTime;
use crate::client::SpiraClient;
use crate::{json_value, parse_date, SpiraError};

#[derive(Clone, Debug, Getters)]
pub struct Requirement {
    id: u64,
    indent_level: Option<Arc<str>>,
    status: RequirementStatus,
    author_id: u64,
    owner_id: Option<u64>,
    importance: Option<RequirementImportance>,
    release_id: Option<u64>,
    /// The ID of the component the requirement is a part of
    component_id: Option<u64>,
    name: Arc<str>,
    description: Option<Arc<str>>,
    creation_date: OffsetDateTime,
    last_updated_date: OffsetDateTime,
    /// Is this a summary requirement or not
    summary: bool,
    /// The version number string of the release that the requirement is scheduled for
    release_version_number: Option<Arc<str>>,
    project_id: u64,
    custom_properties: Option<Map<String, Value>>
}

impl Requirement {
    pub(crate) fn requirements_from_json(json_string: &str, client: &SpiraClient) -> Result<Vec<Requirement>, SpiraError> {
        let Ok(requirements_json): Result<serde_json::Value,_> = serde_json::from_str(json_string) else {
            return Err(SpiraError::JSONParsingError(format!("Requirements JSON is invalid: {}", json_string)))
        };

        let Some(requirements_array) = requirements_json.as_array() else {
            return Err(SpiraError::JSONParsingError(format!("Requirements JSON is not an array: {}", json_string)))
        };

        let mut requirements_vec = Vec::new();

        for project_json in requirements_array {
            let project = Requirement::requirement_from_json(&project_json.to_string(), client)?;
            requirements_vec.push(project);
        }

        return Ok(requirements_vec);
    }


    // Implement this https://stackoverflow.com/questions/63306229/how-to-pass-options-to-rusts-serde-that-can-be-accessed-in-deserializedeseria
    pub(crate) fn requirement_from_json(json_string: &str, client: &SpiraClient) -> Result<Requirement, SpiraError> {
        let mut deserializer = serde_json::Deserializer::new(serde_json::de::StrRead::new(json_string));
        Ok(RequirementDeserializer { client }.deserialize(&mut deserializer)?)
    }
}

struct RequirementDeserializer<'a> {
    client: &'a SpiraClient,
}

impl<'de> DeserializeSeed<'de> for RequirementDeserializer<'_>
{
    type Value = Requirement;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where D: Deserializer<'de>,
    {

        let r = &serde_json::Value::deserialize(deserializer)?;

        let requirement = Requirement {
            id: json_value("RequirementId", r).as_u64().unwrap(),
            indent_level: match json_value("RequirementId", r).as_str() {
                Some(s) => Some(s.into()),
                None => None,
            },
            status: RequirementStatus::from_repr(json_value("StatusId", r).as_u64().unwrap() as usize).unwrap(),
            author_id: json_value("AuthorId", r).as_u64().unwrap(),
            owner_id: json_value("OwnerId", r).as_u64(),
            importance: match json_value("ImportanceId", r).as_u64() {
                Some(n) => RequirementImportance::from_repr(n as usize),
                None => None,
            },
            release_id: json_value("ReleaseId", r).as_u64(),
            component_id: json_value("ComponentId", r).as_u64(),
            name: json_value("Name", r).as_str().unwrap().into(),
            description: match json_value("Description", r).as_str() {
                Some(s) => Some(s.into()),
                None => None,
            },
            creation_date: parse_date(json_value("CreationDate", r).as_str().unwrap()),
            last_updated_date: parse_date(json_value("LastUpdateDate", r).as_str().unwrap()),
            summary: json_value("Summary", r).as_bool().unwrap(),
            release_version_number: match json_value("ReleaseVersionNumber", r).as_str() {
                Some(s) => Some(s.into()),
                None => None,
            },
            project_id: json_value("ProjectId", r).as_u64().unwrap(),
            custom_properties: match json_value("CustomProperties", r).as_object() {
                Some(o) => Some(o.clone()),
                None => None,
            }
        };

        Ok(requirement)
    }
}

#[derive(Clone, Copy, Debug, FromRepr)]
pub enum RequirementStatus {
    Requested = 1,
    Planned = 2,
    InProgress = 3,
    Developed = 4,
    Accepted = 5,
    Rejected = 6,
    Evaluated = 7,
    Obsolete = 8,
    Tested = 9,
    Completed = 10,
}

#[derive(Debug, Clone, Copy, FromRepr)]
pub enum RequirementType {
    Need = 1,
    Feature = 2,
    UseCase = 3,
    UserStory = 4,
    Quality = 5,
    DesignElement = 6,
}

#[derive(Debug, Clone, Copy, FromRepr)]
pub enum RequirementImportance {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
}