use super::{Architecture, License};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub release: u32,
    pub description: Option<String>,
    pub architecture: Vec<Architecture>,
    pub licenses: Vec<License>,
    pub project_url: Vec<String>,
    pub project_ownder: Vec<String>,
}
