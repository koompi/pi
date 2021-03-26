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
mod security;
mod source;
mod source_database;
mod statics;
// Local
pub use application::Application;
pub use architecture::Architecture;
pub use bin_database::{BinDatabase, BinRepo};
pub use build_file::BuildFile;
pub use config::{Configuration, RepoMeta};
pub use dependency::Dependency;
pub use deployment::Deployment;
pub use function::Function;
pub use help::help;
pub use license::License;
pub use metadata::Metadata;
pub use security::Security;
pub use source::Source;
pub use source_database::SourceDatabase;
pub use statics::*;
pub use utils::{download_http, prepare_bases};

// External
use colored::Colorize;
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
        CACHE_DIR.to_path_buf(),
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

        let address = format!("{}/{}.db", &repo.static_address, &repo.name);
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
            } else {
                run_depgraph.register_node(app.metadata.name.clone());
                opt_depgraph.register_node(app.metadata.name.clone());
                buid_depgraph.register_node(app.metadata.name.clone());
                test_depgraph.register_node(app.metadata.name.clone());
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
                    target_package
                        .build_all(&run_depgraph, &repo_config, &db)
                        .await;
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
                            let files: Vec<PathBuf> =
                                pkgs.iter().skip(1).map(|p| PathBuf::from(p)).collect();
                            db.install_files(&run_depgraph, &repo_config, files.to_vec())
                                .await
                                .unwrap();
                        }
                        _ => {
                            db.install(&run_depgraph, &repo_config, pkgs.to_vec())
                                .await
                                .unwrap();
                        }
                    }
                } else {
                    help("install");
                }
            }
            "r" | "remove" | "-r" | "--remove" => {
                if let Some(pkgs) = packages {
                    if !pkgs.is_empty() {
                        for pkg in pkgs.iter() {
                            if let Some(app) = Application::is_installed(&pkg) {
                                let app_dir: PathBuf = LOCAL_DIR.to_owned().join(&pkg);
                                let files = app.files.clone();
                                files.iter().for_each(|file| {
                                    let f: Vec<String> = file
                                        .split(" ")
                                        .into_iter()
                                        .map(|a| a.to_string())
                                        .collect();
                                    let fi = f[0].clone();
                                    std::fs::remove_file(ROOT_DIR.join(&fi)).unwrap()
                                });

                                std::fs::remove_dir_all(app_dir).unwrap();
                            }
                        }
                    } else {
                        help("remove");
                    }
                } else {
                    help("remove");
                }
            }
            "s" | "search" | "-s" | "--search" => {
                if let Some(pkgs) = packages {
                    if !pkgs.is_empty() {
                        for p in pkgs.iter() {
                            let res = db.find(&repo_config, p);
                            if let Some(app) = res {
                                println!("{:#?}", app);
                            } else {
                                println!("{} is not found", p.red().bold());
                            }
                        }
                    } else {
                        help("search");
                    }
                } else {
                    help("search");
                }
            }
            "u" | "update" | "-u" | "--update" => db.update(&run_depgraph, &repo_config).await,
            _ => help("all"),
        }
    } else {
        help("all")
    }

    Ok(())
}
