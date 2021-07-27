use crate::statics::*;
use crate::BuildFile;
use num_cpus;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, io::ErrorKind};
use subprocess::{Exec, ExitStatus};
#[derive(Clone, Debug, Default, Deserialize, Serialize)]

pub struct Function {
    pub commands: Vec<String>,
}

impl Function {
    pub fn exec(&self, pkgdata: &BuildFile) -> Result<(), Box<dyn Error>> {
        // pub fn exec(&self, pkgdata: &BuildFile) -> Result<(), Box<dyn Error>> {
        // Commands to  execute
        // let mut commands = self.commands.clone();
        let mut commands = Vec::new();
        // commands.push(String::from("bash"));
        commands.push(String::from("set -e"));
        commands.push(String::from("fakeroot"));
        commands.push(String::from("echo $SHELL"));
        commands.append(self.commands.clone().as_mut());
        commands.push(String::from("exit"));
        let cmds = &self.commands.join("\n").to_string();

        // Envronment varialbes
        let basedir = CWD_DIR.to_str().unwrap();
        let srcdir = SRC_DIR.to_str().unwrap();
        let pkgdir = PKG_DIR.to_str().unwrap();
        let pkgname = &pkgdata.metadata.name;
        let pkgver = &pkgdata.metadata.version;
        let pkgrel = &pkgdata.metadata.release;

        let num = num_cpus::get();
        env::set_var("MAKEFLAGS", &format!("-j {}", num));
        env::set_var("PKGNAME", pkgname);
        env::set_var("PKGVER", pkgver);
        env::set_var("PKGREL", pkgrel.to_string());
        env::set_var("BASEDIR", basedir);
        env::set_var("SRCDIR", srcdir);
        env::set_var("PKGDIR", pkgdir);
        env::set_var("SHELL", "/bin/bash");
            
        // Exec::cmd("bash").arg("-c").args(&commands).join().unwrap();
        match Exec::shell(cmds).join() {
            Ok(ex) => match ex.success() {
                true => Ok(()),
                false => match ex {
                    ExitStatus::Exited(e) => Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        format!("Process exited with code: {}", e),
                    ))),
                    ExitStatus::Signaled(e) => Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        format!("Process exited with code: {}", e),
                    ))),
                    ExitStatus::Other(e) => Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        format!("Process exited with code: {}", e),
                    ))),
                    ExitStatus::Undetermined => Err(Box::new(std::io::Error::new(
                        ErrorKind::Other,
                        "Undetermined",
                    ))),
                },
            },
            Err(e) => Err(Box::new(e)),
        }






        // match run(cmd, basedir, srcdir, pkgdir, &pkgname, &pkgver, *pkgrel) {
        //     Ok(d) => {
        //         d.for_each(|res| {
        //             println!("{}", res);
        //         });
        //     }
        //     Err(e) => println!("{}", e.to_string().on_red()),
        // }
        // Ok(())
    }
}

// #[shell(cmd = "fakeroot sh -c $MODULE")]
// pub fn run(
//     module: &str,
//     basedir: &str,
//     srcdir: &str,
//     pkgdir: &str,
//     pkgname: &str,
//     pkgver: &str,
//     pkgrel: u32,
// ) -> Result<impl Iterator<Item = String>, Box<Error>> {
//     ""
// }
