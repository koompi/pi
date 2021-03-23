#![allow(
    unused_variables,
    unused_imports,
    dead_code,
    unused_mut,
    unused_assignments
)]
pub mod utils;

mod application;
mod architecture;
mod bin_database;
mod build_file;
mod config;
mod dependency;
mod deployment;
mod function;
mod help;
mod license;
mod metadata;
mod operations;
mod security;
mod source;
mod source_database;
mod statics;
// Local
pub use application::Application;
pub use architecture::Architecture;
pub use bin_database::{BinDatabase, BinRepo};
pub use build_file::BuildFile;
use colored::Colorize;
pub use config::{Configuration, RepoMeta};
pub use dependency::Dependency;
pub use deployment::Deployment;
pub use function::Function;
use help::help;
pub use license::License;
pub use metadata::Metadata;
pub use security::Security;
pub use source::Source;
pub use statics::*;
use utils::{download_http, prepare_bases};

// External
use solvent::DepGraph;
use std::{env, fs::File, path::PathBuf};
use tokio::{fs, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // prepare directories
    prepare_bases(vec![
        #[cfg(debug_assertions)]
        CWD_DIR.to_path_buf(),
        #[cfg(debug_assertions)]
        ROOT_DIR.to_path_buf(),
        SRC_DIR.to_path_buf(),
        PKG_DIR.to_path_buf(),
        LIB_DIR.to_path_buf(),
        LOCAL_DIR.to_path_buf(),
        SYNC_DIR.to_path_buf(),
        CONF_DIR.to_path_buf(),
    ])
    .unwrap();
    // preper config file if running for the first time
    if !CONF_FILE.as_path().exists() {
        let mut file = File::create(CONF_FILE.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &Configuration::gen()).unwrap()
    }
    // dependencies graph
    let mut run_depgraph: DepGraph<String> = DepGraph::new();
    let mut opt_depgraph: DepGraph<String> = DepGraph::new();
    let mut buid_depgraph: DepGraph<String> = DepGraph::new();
    let mut test_depgraph: DepGraph<String> = DepGraph::new();
    // local database
    let mut db: BinDatabase = BinDatabase::new();
    // read the config
    let rdr = File::open(CONF_FILE.as_path()).unwrap();
    let repo_config: Configuration = serde_yaml::from_reader(rdr).unwrap();
    // let database
    // check if all repo listed in config file existed or download it
    println!("{}", "PREPARING DATABASE".green());
    for repo in repo_config.repos.iter() {
        let db_file_path = SYNC_DIR.join(format!("{}.db", &repo.name));

        let address = format!("{}/{}.db", &repo.address, &repo.name);
        if !db_file_path.exists() {
            download_http(
                db_file_path.to_str().unwrap(),
                &format!("{}.db", &repo.name),
                &address,
            )
            .await
            .unwrap();
        }
        let repo_file = File::open(db_file_path).unwrap();
        let repo_data: BinRepo = serde_yaml::from_reader(repo_file).unwrap();

        db.repos.insert(repo.name.clone(), repo_data);
    }

    for (_, repo) in db.repos.iter() {
        for (_, app) in repo.applications.iter() {
            if let Some(deps) = &app.dependencies {
                // runtime dependencies
                if let Some(rd) = &deps.run_dependencies {
                    if rd.is_empty() {
                        run_depgraph.register_node(app.metadata.name.clone());
                    } else {
                        run_depgraph.register_dependencies(app.metadata.name.clone(), rd.to_vec())
                    }
                }
                // optional dependencies
                if let Some(od) = &deps.opt_dependencies {
                    if od.is_empty() {
                        opt_depgraph.register_node(app.metadata.name.clone());
                    } else {
                        opt_depgraph.register_dependencies(app.metadata.name.clone(), od.to_vec())
                    }
                }
                // build dependencies
                if let Some(bd) = &deps.build_dependencies {
                    if bd.is_empty() {
                        buid_depgraph.register_node(app.metadata.name.clone());
                    } else {
                        buid_depgraph.register_dependencies(app.metadata.name.clone(), bd.to_vec())
                    }
                }
                // test dependencies
                if let Some(td) = &deps.test_dependencies {
                    if td.is_empty() {
                        test_depgraph.register_node(app.metadata.name.clone());
                    } else {
                        test_depgraph.register_dependencies(app.metadata.name.clone(), td.to_vec())
                    }
                }
            }
        }
    }

    let args: Vec<String> = env::args_os()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();
    if args.len() > 1 {
        let verb = &args[1];

        let packages: Option<Vec<String>> = if &args.len() >= &(3 as usize) {
            Some(args[2..].to_vec())
        } else {
            None
        };

        match verb.as_ref() {
            "b" | "build" | "-b" | "--build" => {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    println!("{:?}", ps)
                } else {
                    let target_package: BuildFile =
                        BuildFile::from_file(PKG_FILE.to_path_buf()).unwrap();
                    target_package.build_all().await;
                }
            }
            "g" | "generate" | "-g" | "--generate" => {
                let bf = BuildFile::new();
                let file = File::create("pkgbuild.yml").unwrap();
                serde_yaml::to_writer(file, &bf).unwrap();
            }
            "i" | "install" | "-i" | "--install" => {
                if let Some(pkgs) = packages {
                    let local: String = pkgs[0].clone();
                    match local.as_ref() {
                        "-f" | "--file" => {
                            let files: Vec<String> = pkgs[1..].to_vec();
                            // install(files, true).await;
                        }
                        _ => {
                            // install(pkgs, false).await;
                            pkgs.iter().for_each(|pkg| {
                                if let Some(app) = Application::is_installed(&pkg) {
                                    println!("{:#?}", &app)
                                }
                            });
                        }
                    }
                } else {
                    help("install");
                }
            }
            "r" | "remove" | "-r" | "--remove" => {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();

                    println!("{:?}", ps)
                } else {
                    help("remove");
                }
            }
            "u" | "update" | "-u" | "--update" => {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    println!("{:?}", ps)
                } else {
                    help("update");
                }
            }
            _ => help("all"),
        }
    } else {
        help("all")
    }

    Ok(())
}
