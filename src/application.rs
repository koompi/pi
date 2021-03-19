use super::{Dependency, Deployment, Function, Metadata, Security, Source};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    pub metadata: Metadata,
    pub security: Option<Security>,
    pub dependencies: Option<Dependency>,
}
