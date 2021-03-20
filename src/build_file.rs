use crate::source;

use super::{
    statics::{MANI_FILE, PKG_DIR, PKG_FILE, SRC_DIR},
    utils::{create_archive, download_git, download_http, read_to_vec_u8},
    Application, Dependency, Deployment, Function, Metadata, Security, Source,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use url::Url;
use walkdir::WalkDir;
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BuildFile {
    pub metadata: Metadata,
    pub sources: Option<Vec<Source>>,
    pub security: Option<Security>,
    pub dependencies: Option<Dependency>,
    pub prepare: Option<Function>,
    pub build: Option<Function>,
    pub check: Option<Function>,
    pub package: Function,
    pub deployment: Option<Deployment>,
}

impl BuildFile {
    pub fn new() -> Self {
        Self {
            metadata: Metadata::default(),
            sources: Some(vec![Source::GIT {
                address: String::from("git://github.com/calamares/calamares"),
                save_as: String::from("calamares"),
            }]),
            security: None,
            dependencies: None,
            prepare: None,
            build: None,
            check: None,
            package: Function::default(),
            deployment: None,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        match path.exists() {
            false => Err(Error::new(
                ErrorKind::NotFound,
                "No pkgbuild.yml found in current directory",
            )),
            true => {
                let file = File::open(PKG_FILE.display().to_string())
                    .expect("Unable to read pkgbuild.yml");
                match serde_yaml::from_reader(file) {
                    Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
                    Ok(pkg) => Ok(pkg),
                }
            }
        }
    }

    pub fn from(&mut self, path: PathBuf) {
        let data = Self::from_file(path).unwrap();
        *self = data;
    }

    pub fn check_makedepends(&self) {
        if let Some(deps) = &self.dependencies {
            if let Some(build_deps) = &deps.build_dependencies {
                if !build_deps.is_empty() {
                    // Check is make deps installed
                    println!("{:?}", build_deps);
                }
            }
        }
    }

    pub fn build(&self) {
        let name = &self.metadata.name;
        let version = &self.metadata.version;
        let release = self.metadata.release;
        if let Some(prepare_script) = &self.prepare {
            println!("{}", "PREPARING BUILD".green().bold());
            prepare_script.exec(&name, &version, release).unwrap();
        }
        if let Some(build_script) = &self.build {
            println!("{}", "RUNNING BUILD".green().bold());
            build_script.exec(&name, &version, release).unwrap();
        }
        if let Some(check_script) = &self.check {
            println!("{}", "CHECKING BUILD".green().bold());
            check_script.exec(&name, &version, release).unwrap();
        }
        println!("{}", "PACKING BUILD".green().bold());
        self.package.exec(&name, &version, release).unwrap();
    }

    pub async fn pull_one(&self, app_name: &str, path_name: &str, source_address: &str) {
        download_http(path_name, app_name, source_address)
            .await
            .unwrap()
    }
    pub async fn pull_all(&self) {
        if let Some(sources) = &self.sources {
            if !sources.is_empty() {
                for source in sources.iter() {
                    source.pull().await;
                }
            }
        }
    }
    // pub async fn pull_all(&self) {
    //     if let Some(sources) = &self.sources {
    //         for source in sources {
    //             let parsed_url = Url::parse(source).expect("Unable to parse URL");
    //             let file_name = &parsed_url
    //                 .path_segments()
    //                 .unwrap()
    //                 .last()
    //                 .expect("Cannot get file name for URL");
    //             let file_path = SRC_DIR.join(file_name);

    //             match parsed_url.scheme() {
    //                 "git" => {
    //                     println!("Cloning {}", &parsed_url.to_string());
    //                     download_git(parsed_url.to_string().as_ref(), file_path.to_str().unwrap())
    //                 }
    //                 "http" | "https" => {
    //                     self.pull_one(
    //                         file_name,
    //                         &file_path.to_str().unwrap().to_string(),
    //                         &parsed_url.to_string(),
    //                     )
    //                     .await;
    //                 }
    //                 _ => {
    //                     println!("Unsupported URL")
    //                 }
    //             }
    //         }
    //     }
    // }

    pub fn archive_name(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.metadata.name, self.metadata.version, self.metadata.release, "x86_64"
        )
    }

    pub fn gen_file_list(&self) -> Vec<String> {
        if MANI_FILE.exists() {
            std::fs::remove_file(MANI_FILE.to_path_buf()).unwrap()
        }

        let mut files: Vec<String> = Vec::new();

        for entry in WalkDir::new(PKG_DIR.to_path_buf()).min_depth(1) {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() {
                let shahash = read_to_vec_u8(&entry.path());

                let mut hasher = Sha256::new();
                hasher.update(&shahash);

                let mut buf = entry
                    .path()
                    .display()
                    .to_string()
                    .trim_start_matches(PKG_DIR.to_str().unwrap())
                    .trim_start_matches("/")
                    .to_string();
                buf.push_str(&format!(" {:x}", hasher.finalize()));
                files.push(buf);
            }
        }

        files
    }
    pub fn to_app(&self) -> Application {
        Application {
            metadata: self.metadata.clone(),
            security: self.security.clone(),
            dependencies: self.dependencies.clone(),
        }
    }

    pub fn create_package(&self) {
        create_archive(&self, PKG_DIR.to_path_buf())
    }
}
