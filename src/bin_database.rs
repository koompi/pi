use super::Application;
use crate::utils::{decompress_all, decompress_zstd, download_http, extract_archive};
use crate::{Configuration, CACHE_DIR, ROOT_DIR, SUFFIX_APP};
use colored::Colorize;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use serde_yaml;
use solvent::DepGraph;
use std::collections::HashMap;
use std::fs::File;
use std::time::SystemTime;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinDatabase {
    pub repos: HashMap<String, BinRepo>,
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
            if let Some(repo_address) = config.get_address(&repo_name) {
                if let Some(application) = repo.applications.get(app) {
                    let package_name = &application.archive_name();
                    let package_address: Url = Url::parse(&repo_address).unwrap();
                    let full_address = package_address.join(package_name).unwrap();

                    res = Some(TargetPackage {
                        repo: repo_name.clone(),
                        package_address: full_address.to_string(),
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

    // pub fn install_one(&self, app: &str) {
    //     match self.find(app) {
    //         Some(a) => {
    //             // decompress_zstd(arg_file).unwrap();
    //             // extract_archive(arg_file, &dest.to_str().unwrap()).unwrap();
    //             // list_installed();
    //         }
    //         None => {
    //             println!("Application not found: {}", app.red())
    //         }
    //     }
    // }

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
        let mut to_downloads: Vec<String> = Vec::new();

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
                        to_install_name.push(node.unwrap().to_string())
                    }
                }
            }

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
                        let file_path =
                            CACHE_DIR.to_path_buf().join(&target.package.archive_name());
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
                        let file_path =
                            CACHE_DIR.to_path_buf().join(&target.package.archive_name());
                        let target_str = file_path.to_str().unwrap().to_string();

                        decompress_zstd(&target_str).unwrap();
                        extract_archive(
                            &target_str.trim_end_matches(&SUFFIX_APP.to_string()),
                            ROOT_DIR.to_str().unwrap(),
                        )
                        .unwrap();

                        pb.inc(1);
                    }
                    pb.finish();
                }
                // Note

                // Completed getting package url

                // 1. load package info
                // 2. download
                // 3. extract
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BinRepo {
    pub applications: HashMap<String, Application>,
    pub date: SystemTime,
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
            for (name, app) in self.applications.iter() {
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
            for (name, app) in self.applications.iter() {
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
            for (name, app) in self.applications.iter() {
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
            for (name, app) in self.applications.iter() {
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
