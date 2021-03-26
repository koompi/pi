use super::{
    statics::{LOCAL_DIR, MANI_FILE},
    Dependency, Deployment, Function, Metadata, Security, Source,
};
use semver::Version;

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Application {
    pub metadata: Metadata,
    pub security: Option<Security>,
    pub dependencies: Option<Dependency>,
    pub files: Vec<String>,
}

impl Application {
    pub fn archive_name(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.metadata.name, self.metadata.version, self.metadata.release, "x86_64"
        )
    }
    pub fn write(&self) -> Result<(), std::io::Error> {
        let f = MANI_FILE.to_path_buf();
        let file = File::create(f);
        match file {
            Ok(f) => match serde_yaml::to_writer(f, &self) {
                Ok(_) => Ok(()),
                Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
            },
            Err(e) => Err(e),
        }
    }

    pub fn is_installed(name: &str) -> Option<Application> {
        let path: PathBuf = PathBuf::from(LOCAL_DIR.join(&format!("{}/manifest.yml", name)));
        if path.exists() {
            let file = File::open(path.as_path()).unwrap();

            let app: Application = serde_yaml::from_reader(file).unwrap();
            Some(app)
        } else {
            None
        }
    }
}
