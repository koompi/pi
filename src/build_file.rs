use crate::{source, Configuration};

use crate::{
    statics::{MANI_FILE, PKG_DIR, PKG_FILE, SRC_DIR},
    utils::{create_archive, decompress_all, download_git, download_http, read_to_vec_u8},
    Application, BinDatabase, Dependency, Deployment, Function, Metadata, Security, Source,
    SourceDatabase,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use solvent::DepGraph;
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
            sources: Some(vec![Source {
                address: String::from("git://github.com/calamares/calamares"),
                save_as: String::from("calamares"),
                extract: false,
                extract_to: None,
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

    pub async fn check_build_dependencies(
        &self,
        rd: &DepGraph<String>,
        config: &Configuration,
        bdb: &BinDatabase,
    ) {
        let mut not_installed_deps: Vec<String> = Vec::new();
        let mut to_install_deps: Vec<String> = Vec::new();
        let mut missing_deps: Vec<String> = Vec::new();

        if let Some(deps) = &self.dependencies {
            if let Some(build_deps) = &deps.build_dependencies {
                if !build_deps.is_empty() {
                    // 1. Check is make deps installed
                    for bd in build_deps.iter() {
                        if let None = Application::is_installed(bd) {
                            not_installed_deps.push(bd.to_string())
                        }
                    }

                    if !not_installed_deps.is_empty() {
                        for bdep in not_installed_deps.iter() {
                            match bdb.find(config, bdep) {
                                Some(a) => to_install_deps.push(bdep.clone()),
                                None => missing_deps.push(bdep.clone()),
                            }
                        }
                    }

                    if missing_deps.is_empty() {
                        if !to_install_deps.is_empty() {
                            // for app in to_install_deps.iter() {
                            //     // install the dep there
                            // }
                            bdb.install(&rd, config, to_install_deps).await.unwrap();
                        }
                    } else {
                        println!(
                            "Unable to find {singplu}: {list}",
                            singplu = if missing_deps.len() > 1 {
                                "dependencies"
                            } else {
                                "dependency"
                            },
                            list = missing_deps.join(", ")
                        );

                        std::process::exit(1);
                    }
                    // 2. Check if exists in db => Install
                    // 3.
                    // println!("{:?}", build_deps);
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
            prepare_script.exec(&self).unwrap();
        }
        if let Some(build_script) = &self.build {
            println!("{}", "RUNNING BUILD".green().bold());
            build_script.exec(&self).unwrap();
        }
        if let Some(check_script) = &self.check {
            println!("{}", "CHECKING BUILD".green().bold());
            check_script.exec(&self).unwrap();
        }
        println!("{}", "PACKING BUILD".green().bold());
        self.package.exec(&self).unwrap();
    }

    pub async fn pull_one(&self, app_name: &str, path_name: &str, source_address: &str) {
        download_http(path_name, app_name, source_address)
            .await
            .unwrap()
    }

    pub async fn pull_all(&self) -> Result<(), anyhow::Error> {
        if let Some(sources) = &self.sources {
            if !sources.is_empty() {
                for source in sources.iter() {
                    let parsed_url = Url::parse(&source.address).expect("Unable to parse URL");
                    let save_as = SRC_DIR.join(&source.save_as);
                    let extract = source.extract;
                    let extract_to: Option<PathBuf> = if extract {
                        Some(SRC_DIR.join(source.extract_to.as_ref().unwrap()))
                    } else {
                        None
                    };

                    match parsed_url.scheme() {
                        "git" => {
                            println!("Cloning {}", &parsed_url.to_string());
                            download_git(parsed_url.to_string().as_ref(), save_as.to_str().unwrap())
                        }
                        "http" | "https" => download_http(
                            save_as.to_str().unwrap(),
                            &source.save_as,
                            &parsed_url.to_string(),
                        )
                        .await
                        .unwrap(),
                        _ => {
                            println!("Unsupported URL")
                        }
                    }

                    if extract {
                        if let Some(ext) = extract_to {
                            decompress_all(&save_as.to_str().unwrap(), ext.to_str().unwrap())
                                .unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn build_all(
        &self,
        rd: &DepGraph<String>,
        config: &Configuration,
        bdb: &BinDatabase,
    ) {
        &self.check_build_dependencies(&rd, &config, &bdb).await;
        &self.pull_all().await;
        &self.build();
        &self.to_app().write().unwrap();
        &self.create_package();
    }

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
            files: self.gen_file_list(),
        }
    }

    pub fn create_package(&self) {
        create_archive(&self, PKG_DIR.to_path_buf())
    }
}
