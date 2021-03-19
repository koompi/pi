use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Deployment {
    KOOMPI { api_key: String },
    KUMANDRA { api_key: String },
}
