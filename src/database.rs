use super::{Application, BuildFile};
use serde::{Deserialize, Serialize};

pub struct LocalDB {
    applications: Vec<Application>,
}

pub struct OnlineDB {
    applications: Vec<Application>,
}

impl LocalDB {
    pub fn find_by_name(&self, name: String) -> Option<BuildFile> {
        self.pkgbuilds
            .iter()
            .find(|build_file| build_file.metadata.name == name)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BuildFileDB {
    pkgbuilds: Vec<BuildFile>,
}

impl BuildFileDB {
    pub fn find_by_name(&self, name: String) -> Option<BuildFile> {
        self.pkgbuilds
            .iter()
            .find(|build_file| build_file.metadata.name == name)
    }

    pub fn find_dependencies(&self, name: String, initial: Option<Vec<String>>) -> Vec<String> {
        let mut data: Vec<String> = Vec::new();
        // get all run time dependencies
        if let Some(build_file) = self.find_by_name(name) {
            if let Some(dependencies) = build_file.dependencies {
                if let Some(run_dependencies) = dependencies.run_dependencies {
                    if !run_dependencies.is_empty() {
                        // data.append(run_dependencies)
                        run_dependencies.iter().for_each(f)
                    }
                }
            }
        }
    }
}
