use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub repos: Vec<RepoMeta>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            repos: vec![RepoMeta::default()],
        }
    }
}

impl Configuration {
    pub fn gen() -> Self {
        Self::default()
    }

    pub fn from_file(path: &str) -> Self {
        let file_path = PathBuf::from(path);
        let file = File::open(file_path).unwrap();
        let data = from_reader(file).unwrap();
        data
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoMeta {
    pub name: String,
    pub address: String,
}

impl Default for RepoMeta {
    fn default() -> Self {
        let address = if cfg!(debug_assertions) {
            String::from("http://localhost:3690/core")
        } else {
            String::from("http://dev.koompi.org/core")
        };
        Self {
            name: String::from("core"),
            address,
        }
    }
}
