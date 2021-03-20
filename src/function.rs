use crate::statics::*;
use serde::{Deserialize, Serialize};
use shellfn::shell;
use std::error::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]

pub struct Function {
    pub commands: Vec<String>,
}

impl Function {
    pub fn exec(&self, name: &str, version: &str, release: u32) -> Result<(), Box<dyn Error>> {
        // Commands to  execute
        let mut commands = self.commands.clone();
        commands.push(String::from("exit"));
        let cmd = &self.commands.join("\n").to_string();

        // Envronment varialbes
        let basedir = CWD_DIR.to_str().unwrap();
        let srcdir = SRC_DIR.to_str().unwrap();
        let pkgdir = PKG_DIR.to_str().unwrap();
        // let pkgname = &DATA_PACKAGE.as_ref().unwrap().pkgname;
        // let pkgver = &DATA_PACKAGE.as_ref().unwrap().pkgver;
        // let pkgrel = DATA_PACKAGE.as_ref().unwrap().pkgrel;

        // execute commands
        run(cmd, basedir, srcdir, pkgdir, name, version, release)
            .map(|output| println!("{}", output))
    }
}

#[shell(cmd = "fakeroot sh -c $MODULE")]
pub fn run(
    module: &str,
    basedir: &str,
    srcdir: &str,
    pkgdir: &str,
    pkgname: &str,
    pkgver: &str,
    pkgrel: u32,
) -> Result<String, Box<dyn Error>> {
    ""
}
