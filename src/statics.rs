use lazy_static::*;
use std::{env, path::PathBuf};

#[macro_export]
lazy_static! {
    // Suffixes
    pub static ref SUFFIX_DB: String = String::from(".db");
    pub static ref SUFFIX_APP: String = String::from(".app");

    // Package Manager
    pub static ref ROOT_DIR: PathBuf = root();
    pub static ref LIB_DIR: PathBuf = ROOT_DIR.join("var/lib/store");
    pub static ref LOCAL_DIR: PathBuf = LIB_DIR.join("local");
    pub static ref SYNC_DIR: PathBuf = LIB_DIR.join("sync");
    pub static ref CACHE_DIR: PathBuf = LIB_DIR.join("cache");
    pub static ref CONF_DIR: PathBuf = ROOT_DIR.join("etc/store");

    pub static ref CONF_FILE: PathBuf = CONF_DIR.join("store.conf");
    pub static ref MANI_FILE: PathBuf = PKG_DIR.join("manifest.yml");

    // Package Builder
    pub static ref CWD_DIR: PathBuf = cwd();
    pub static ref SRC_DIR: PathBuf = cwd().join("source");
    pub static ref PKG_DIR: PathBuf = cwd().join("package");

    pub static ref PKG_FILE: PathBuf = cwd().join("pkgbuild.yml");

    // Server
    pub static ref PUB_DIR: PathBuf = ROOT_DIR.join("var/www/");
    pub static ref SERVER_CFG_DIR: PathBuf = ROOT_DIR.join("etc/repo");
    pub static ref SERVER_CFG_FILE: PathBuf = SERVER_CFG_DIR.join("repo.conf");

}

fn root() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs")
    } else {
        PathBuf::from("/")
    }
}

fn cwd() -> PathBuf {
    if cfg!(debug_assertions) {
        env::current_dir().unwrap().join("rootfs/tmp")
    } else {
        env::current_dir().unwrap()
    }
}
