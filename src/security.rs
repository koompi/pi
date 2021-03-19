use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Security {
    pub maintainer: Vec<String>,
    pub md5sum: String,
    pub sha256sum: String,
    pub gpg_public_key: String,
}
