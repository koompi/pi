#![allow(unused_variables, unused_imports, dead_code)]
pub mod utils;

mod application;
mod architecture;
mod build_file;
mod config;
mod database;
mod dependency;
mod deployment;
mod function;
mod help;
mod license;
mod metadata;
mod operations;
mod security;
mod source;
mod statics;
// Local
pub use application::Application;
pub use architecture::Architecture;
pub use build_file::BuildFile;
pub use config::Configuration;
pub use database::Database;
pub use dependency::Dependency;
pub use deployment::Deployment;
pub use function::Function;
use help::help;
pub use license::License;
pub use metadata::Metadata;
pub use security::Security;
pub use source::Source;
pub use statics::*;
use utils::prepare_bases;
// External
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
    // preper config file
    if !CONF_FILE.as_path().exists() {
        let mut file = File::create(CONF_FILE.as_path()).unwrap();
        serde_yaml::to_writer(&mut file, &Configuration::gen()).unwrap()
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
                    // help("build");
                    let target_package: BuildFile =
                        BuildFile::from_file(PKG_FILE.to_path_buf()).unwrap();
                    target_package.build_all().await;

                    // target_package.build();
                    // target_package.to_manifest().write().unwrap();
                    // target_package.create_package();
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
