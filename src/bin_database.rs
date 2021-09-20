use super::Application;
use crate::utils::{decompress_zstd, download_http, extract_archive};
use crate::{Configuration, CACHE_DIR, LOCAL_DIR, ROOT_DIR, SUFFIX_APP, SYNC_DIR};
use colored::Colorize;
use indicatif::ProgressBar;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_yaml;
use solvent::DepGraph;
use std::{collections::HashMap, path::PathBuf};
use std::{fs::remove_file, time::SystemTime};
use std::{fs::File, io::Read};
use url::Url;

use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinDatabase {
    pub repos: HashMap<String, BinRepo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinRepo {
    pub applications: HashMap<String, Application>,
    pub date: SystemTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TargetPackage {
    pub repo: String,
    pub package_address: String,
    pub package: Application,
}

impl BinDatabase {
    pub fn new() -> Self {
        Self {
            repos: HashMap::new(),
        }
    }

    pub fn find(&self, config: &Configuration, app: &str) -> Option<TargetPackage> {
        let mut res: Option<TargetPackage> = None;
        for (repo_name, repo) in self.repos.iter() {
            if let Some(repo_address) = config.get_static_address(&repo_name) {
                if let Some(application) = repo.applications.get(app) {
                    let package_name = format!("{}.app", &application.archive_name());
                    let package_address: Url = Url::parse(&repo_address).unwrap();
                    let full_address = package_address.join(&package_name).unwrap();

                    println!("{:#?}", package_address.as_str());

                    res = Some(TargetPackage {
                        repo: repo_name.clone(),
                        package_address: format!("{}", full_address.to_string()),
                        package: application.clone(),
                    })
                }
            } else {
                println!("Failed to get repo address: {}", &repo_name);
                std::process::exit(1);
            }
        }
        res
    }

    pub async fn install(
        &self,
        rd: &DepGraph<String>,
        repo_config: &Configuration,
        packages: Vec<String>,
    ) -> Result<(), Vec<String>> {
        let mut not_found_packages: Vec<String> = Vec::new();
        let mut to_install_name: Vec<String> = Vec::new();
        let mut to_install: Vec<TargetPackage> = Vec::new();
        let mut missing_from_db: Vec<String> = Vec::new();
        // let to_downloads: Vec<String> = Vec::new();

        for package in packages.iter() {
            if let None = &self.find(repo_config, package) {
                not_found_packages.push(package.to_string());
            }
        }

        if !not_found_packages.is_empty() {
            println!(
                "Unable to find {singplu}: {list}",
                singplu = if not_found_packages.len() > 1 {
                    "packages"
                } else {
                    "package"
                },
                list = not_found_packages.join(", ")
            );

            std::process::exit(1);
        } else {
            for package in packages.iter() {
                if let Ok(nodes) = rd.dependencies_of(package) {
                    for node in nodes {
                        // to_install_name.push(node.unwrap().to_string())
                        if let Ok(node) = node {
                            to_install_name.push(node.to_string())
                        }
                    }
                }
            }
            println!("{:?}", to_install_name);
            if !to_install_name.is_empty() {
                to_install_name.dedup();

                for name in to_install_name.iter() {
                    if let Some(pkg) = self.find(&repo_config, name) {
                        to_install.push(pkg.clone())
                    } else {
                        missing_from_db.push(name.to_string())
                    }
                }

                if !missing_from_db.is_empty() {
                    println!(
                        "Following {singplu} are missing from database: : {list}",
                        singplu = if missing_from_db.len() > 1 {
                            "packages"
                        } else {
                            "package"
                        },
                        list = missing_from_db.join(", ")
                    );
                } else {
                    // Download packages
                    println!("{}", "DOWNLOADING PACKAGES".green());

                    for target in to_install.iter() {
                        let file_path = CACHE_DIR
                            .to_path_buf()
                            .join(format!("{}.app", &target.package.archive_name()));
                        download_http(
                            file_path.to_str().unwrap(),
                            &target.package.metadata.name,
                            &target.package_address,
                        )
                        .await
                        .unwrap();
                    }
                    println!("{}", "INSTALLING PACKAGES".green());
                    let pb = ProgressBar::new(to_install.len() as u64);
                    for target in to_install.iter() {
                        let file_path = CACHE_DIR
                            .to_path_buf()
                            .join(format!("{}.app", &target.package.archive_name()));
                        let target_str = file_path.to_str().unwrap().to_string();

                        decompress_zstd(&target_str).unwrap();
                        extract_archive(
                            &target_str.trim_end_matches(&SUFFIX_APP.to_string()),
                            ROOT_DIR.to_str().unwrap(),
                        )
                        .unwrap();

                        std::fs::remove_file(&target_str).unwrap();
                        std::fs::remove_file(&target_str.trim_end_matches(&SUFFIX_APP.to_string()))
                            .unwrap();
                        pb.inc(1);
                    }
                    pb.finish();
                }
            }
        }

        Ok(())
    }

    pub async fn install_files(
        &self,
        _rd: &DepGraph<String>,
        _repo_config: &Configuration,
        packages: Vec<PathBuf>,
    ) -> std::io::Result<()> {
        let mut tar_files: Vec<String> = Vec::new();

        for package in packages.iter() {
            decompress_zstd(&package.to_str().unwrap()).unwrap();
            tar_files.push(
                package
                    .to_str()
                    .unwrap()
                    .to_string()
                    .trim_end_matches(".app")
                    .to_string(),
            );
        }
        let mut valid_packages: Vec<String> = Vec::new();
        for tar_file in tar_files.iter() {
            let file = File::open(tar_file).unwrap();
            let mut archive = tar::Archive::new(file);

            match archive.entries() {
                Ok(mut entries) => {
                    let res = entries.find(|f| {
                        f.as_ref().unwrap().path().unwrap().to_str().unwrap() == "manifest.yml"
                    });
                    let mut buf = String::new();
                    res.unwrap().unwrap().read_to_string(&mut buf).unwrap();

                    let data: Result<Application, serde_yaml::Error> = serde_yaml::from_str(&buf);
                    match data {
                        Ok(_) => valid_packages.push(tar_file.clone()),
                        Err(e) => {
                            println!("{}", e.to_string());
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    std::process::exit(1);
                }
            }
        }
        println!("{}", "INSTALLING PACKAGES".green());
        let pb = ProgressBar::new(valid_packages.len() as u64);

        for package in valid_packages.iter() {
            extract_archive(&package, ROOT_DIR.to_str().unwrap()).unwrap();
            pb.inc(1);
        }
        pb.finish();

        Ok(())
    }

    pub async fn update(&self, rd: &DepGraph<String>, config: &Configuration) {
        for repo in config.repos.iter() {
            let version = reqwest::get(&repo.update_address)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            let online_version: u64 = version.parse().unwrap();
            let local_version: u64 = self
                .repos
                .get(&repo.name)
                .unwrap()
                .date
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if online_version > local_version {
                let db_file_path = SYNC_DIR.join(format!("{}.db", &repo.name));
                let address = format!("{}/{}.db", &repo.static_address, &repo.name);
                remove_file(&db_file_path).unwrap();
                download_http(
                    db_file_path.to_str().unwrap(),
                    &format!("{}.db", &repo.name),
                    &address,
                )
                .await
                .unwrap();

                let mut to_update: Vec<String> = Vec::new();
                if let Some(apps) = self.list_installed() {
                    if !apps.is_empty() {
                        apps.iter().for_each(|f| {
                            let install_v = Version::parse(&f.metadata.version).unwrap();
                            let let_in_db = self.find(&config, &f.metadata.name).unwrap();
                            let dbv = Version::parse(&let_in_db.package.metadata.version).unwrap();

                            if dbv.gt(&install_v) {
                                to_update.push(f.metadata.name.to_string());
                            }
                        })
                    }
                }
                if !to_update.is_empty() {
                    self.install(rd, &config, to_update).await.unwrap()
                }
            } else {
                println!("{}", "Your system is already up to date.".green())
            }
        }
    }

    pub fn list_installed(&self) -> Option<Vec<Application>> {
        let mut res: Vec<Application> = Vec::new();
        for entry in WalkDir::new(LOCAL_DIR.as_path())
            .min_depth(1)
            .max_depth(1)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        {
            let app_path = entry.unwrap().path().join("manifest.yml");
            if app_path.exists() {
                let file = File::open(app_path).unwrap();
                match serde_yaml::from_reader(file) {
                    Ok(d) => res.push(d),
                    Err(_) => {}
                }
            }
        }

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }
}

impl BinRepo {
    // Create
    pub fn new() -> Self {
        Self {
            applications: HashMap::new(),
            date: SystemTime::now(),
        }
    }

    pub fn from(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let data: BinRepo = serde_yaml::from_reader(file).unwrap();
        data
    }

    // Dependencies
    pub fn get_run_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for (_name, app) in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.run_dependencies {
                        let rdeps = run_deps;
                        depgraph.register_dependencies(name, rdeps.to_vec())
                    }
                } else {
                    depgraph.register_node(name)
                }
            }
        }
        depgraph
    }

    pub fn get_opt_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for (_name, app) in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.opt_dependencies {
                        let rdeps = run_deps;
                        depgraph.register_dependencies(name, rdeps.to_vec())
                    }
                } else {
                    depgraph.register_node(name)
                }
            }
        }
        depgraph
    }

    pub fn get_build_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for (_name, app) in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.build_dependencies {
                        let rdeps = run_deps;
                        depgraph.register_dependencies(name, rdeps.to_vec())
                    }
                } else {
                    depgraph.register_node(name)
                }
            }
        }
        depgraph
    }

    pub fn get_test_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for (_name, app) in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.test_dependencies {
                        let rdeps = run_deps;
                        depgraph.register_dependencies(name, rdeps.to_vec())
                    }
                } else {
                    depgraph.register_node(name)
                }
            }
        }
        depgraph
    }
}
