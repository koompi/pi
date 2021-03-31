use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{fs::File, path::PathBuf};

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

    pub fn get_static_address(&self, name: &str) -> Option<String> {
        if let Some(repo) = &self.repos.iter().find(|repo| repo.name == name) {
            return Some(repo.static_address.clone());
        } else {
            return None;
        }
    }
    pub fn get_update_address(&self, name: &str) -> Option<String> {
        if let Some(repo) = &self.repos.iter().find(|repo| repo.name == name) {
            return Some(repo.update_address.clone());
        } else {
            return None;
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoMeta {
    pub name: String,
    pub static_address: String,
    pub update_address: String,
}

impl Default for RepoMeta {
    fn default() -> Self {
        let static_address = if cfg!(debug_assertions) {
            format!("http://localhost:3690/core/")
        } else {
            format!("http://dev.koompi.org/core/")
        };
        let update_address = if cfg!(debug_assertions) {
            format!("http://localhost:3690/version/core")
        } else {
            format!("http://dev.koompi.org/version/core")
        };
        Self {
            name: String::from("core"),
            static_address,
            update_address,
        }
    }
}
