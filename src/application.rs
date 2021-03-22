use super::{statics::MANI_FILE, Dependency, Deployment, Function, Metadata, Security, Source};
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
}
