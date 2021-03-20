use crate::{
    statics::SRC_DIR,
    utils::{decompress_all, download_git, download_http},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Source {
    HTTP {
        address: String,
        save_as: String,
        extract: bool,
        extract_to: String,
    },
    GIT {
        address: String,
        save_as: String,
    },
    // IPFS {
    //     cid: String,
    //     save_as: String,
    //     extract: bool,
    // },
    // FTP {
    //     address: String,
    //     save_as: String,
    //     extract: String,
    // },
    // RSYNC {
    //     address: String,
    //     ssh_key: Option<String>,
    //     save_as: String,
    //     extract: bool,
    // },
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
            extract_to: String::new(),
        }
    }
}

impl Source {
    pub async fn pull(&self) {
        match self {
            Source::HTTP {
                address,
                save_as,
                extract,
                extract_to,
            } => {
                let target = SRC_DIR.as_path().join(&save_as);
                download_http(target.to_str().unwrap(), save_as, address)
                    .await
                    .unwrap();
                if extract.to_owned() {
                    decompress_all(&target.to_str().unwrap(), extract_to).unwrap();
                }
            }
            Source::GIT { address, save_as } => {
                let target = SRC_DIR.as_path().join(&save_as);
                println!("Cloning {}", &address);
                download_git(address, target.to_str().unwrap());
            }
            _ => {}
        }
    }
}
// Source::FTP {
//     address,
//     save_as,
//     extract,
// } => {
//     use std::io::{self, Write};
//     use std::process::Command;
//     let cmd = Command::new("curl")
//         .args(&[
//             "-P",
//             "-",
//             "-#",
//             "--insecure",
//             address,
//             "-o",
//             save_as,
//             "--user",
//             "anonymous:anonymous",
//         ])
//         .output()
//         .unwrap();
//     std::io::stdout().write_all(&cmd.stdout).unwrap();
//     std::io::stderr().write_all(&cmd.stderr).unwrap();
// }
// Source::IPFS {
//     cid,
//     save_as,
//     extract,
// } => {}
// Source::FILE {
//     path,
//     save_as,
//     extract,
// } => {}
// Source::RSYNC {
//     address,
//     ssh_key,
//     save_as,
//     extract,
// } => {}
