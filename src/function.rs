use crate::statics::*;
use crate::BuildFile;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use shellfn::shell;
use std::error::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]

pub struct Function {
    pub commands: Vec<String>,
}

impl Function {
    pub fn exec(&self, pkgdata: &BuildFile) -> Result<(), Box<dyn Error>> {
        // pub fn exec(&self, pkgdata: &BuildFile) -> Result<(), Box<dyn Error>> {
        // Commands to  execute
        let mut commands = self.commands.clone();
        commands.push(String::from("exit"));
        let cmd = &self.commands.join("\n").to_string();

        // Envronment varialbes
        let basedir = CWD_DIR.to_str().unwrap();
        let srcdir = SRC_DIR.to_str().unwrap();
        let pkgdir = PKG_DIR.to_str().unwrap();
        let pkgname = &pkgdata.metadata.name;
        let pkgver = &pkgdata.metadata.version;
        let pkgrel = &pkgdata.metadata.release;

        match run(cmd, basedir, srcdir, pkgdir, &pkgname, &pkgver, *pkgrel) {
            Ok(d) => {
                d.for_each(|res| {
                    println!("{}", res);
                });
            }
            Err(e) => println!("{}", e.to_string().on_red()),
        }

        Ok(())
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
) -> Result<impl Iterator<Item = String>, Box<Error>> {
    ""
}
