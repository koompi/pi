use super::compress::compress_zstd;
use crate::statics::LOCAL_DIR;
use crate::statics::*;
use crate::utils::prepare::prepare_base;
use crate::{Application, BuildFile};
use std::{
    fs::{create_dir_all, remove_file, File},
    io::{Read, Result},
    path::{Path, PathBuf},
};
use tar::Archive;
use zip::ZipArchive;

pub fn extract_archive(arg_file: &str, dest: &str) -> Result<()> {
    let dest_path: PathBuf = PathBuf::from(&dest);
    if !dest_path.exists() {
        create_dir_all(&dest).unwrap();
    }

    let file = File::open(arg_file)?;
    let mut a = Archive::new(file);

    match a.entries() {
        Ok(all_entries) => Ok(for file in all_entries {
            let mut f = file?;
            let p: String = f.path()?.clone().to_str().unwrap().to_string();
            match p.as_str() {
                "manifest.yml" => {
                    let mut buf: String = String::new();
                    f.read_to_string(&mut buf).unwrap();
                    let manifest: Application = serde_yaml::from_str(&buf).unwrap();
                    let destname = LOCAL_DIR.join(&manifest.metadata.name);
                    prepare_base(destname.clone()).unwrap();
                    serde_yaml::to_writer(File::create(destname.join(&p)).unwrap(), &manifest)
                        .unwrap();
                }
                _ => {
                    f.unpack(Path::new(dest).join(&p))?;
                }
            };
        }),
        Err(e) => return Err(e),
    }
}

pub fn extract_zip(arg_file: &str, dest: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(arg_file)?;
        ZipArchive::new(file)?
    };
    decoder.extract(dest).unwrap();

    Ok(())
}

pub fn create_archive(data: &BuildFile, path: PathBuf) {
    let archive_name = data.archive_name();
    let pkgf = File::create(&archive_name).unwrap();
    let mut tar = tar::Builder::new(pkgf);
    tar.append_dir_all(".", path).unwrap();
    compress_zstd(&archive_name).unwrap();
    remove_file(&archive_name).unwrap();
}
