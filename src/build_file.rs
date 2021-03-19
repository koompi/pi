use super::{Application, Dependency, Deployment, Function, Metadata, Security, Source};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BuildFile {
    pub metadata: Metadata,
    pub sources: Option<Vec<Source>>,
    pub security: Option<Security>,
    pub dependencies: Option<Dependency>,
    pub actions: Option<Vec<Function>>,
    pub deployment: Option<Deployment>,
}

impl BuildFile {
    pub fn to_app(&self) -> Application {
        Application {
            metadata: self.metadata.clone(),
            security: self.security.clone(),
            dependencies: self.dependencies.clone(),
        }
    }
}
