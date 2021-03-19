use crate::utils::{download_git, download_http};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    HTTP {
        address: String,
        save_as: String,
        extract: bool,
    },
    GIT {
        address: String,
        save_as: String,
    },
    IPFS {
        cid: String,
        save_as: String,
        extract: bool,
    },
    FTP {
        address: String,
        save_as: String,
        extract: String,
    },
    RSYNC {
        address: String,
        ssh_key: Option<String>,
        save_as: String,
        extract: bool,
    },
    FILE {
        path: String,
        save_as: String,
        extract: bool,
    },
}

impl Default for Source {
    fn default() -> Self {
        Self::HTTP {
            address: String::new(),
            save_as: String::new(),
            extract: false,
        }
    }
}

impl Source {
    pub async fn perform(&self) {
        match self {
            Source::HTTP {
                address,
                save_as,
                extract,
            } => {
                download_http(save_as, save_as, address).await.unwrap();
            }
            Source::GIT { address, save_as } => {
                download_git(address, save_as);
            }
            Source::FTP {
                address,
                save_as,
                extract,
            } => {
                use std::process::Command;
                Command::new("curl")
                    .args(&[
                        "-P",
                        "-",
                        "--insecure",
                        address,
                        "-o",
                        save_as,
                        "--user",
                        "anonymous:anonymous",
                    ])
                    .spawn()
                    .expect("Fialed to download");
            }
            Source::IPFS {
                cid,
                save_as,
                extract,
            } => {}
            Source::FILE {
                path,
                save_as,
                extract,
            } => {}
            Source::RSYNC {
                address,
                ssh_key,
                save_as,
                extract,
            } => {}
        }
    }
}
