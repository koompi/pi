use crate::statics::*;
use crate::utils::{extract_archive, extract_zip};
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{create_dir_all, File},
    io::*,
    path::PathBuf,
    process::{Command, Stdio},
    str,
};
use tar::Archive;
use xz2::read::XzDecoder;
use zstd::Decoder;

pub fn decompress_all(source: &str, dest: &str) -> Result<()> {
    let dest_path: PathBuf = PathBuf::from(&dest);
    if !dest_path.exists() {
        create_dir_all(&dest).unwrap();
    }

    let file_path: PathBuf = PathBuf::from(source);

    match file_path.extension().unwrap().to_str().unwrap() {
        "bz2" => {
            let file = File::open(source)?;
            let tar = BzDecoder::new(file);
            let mut archive = Archive::new(tar);

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                file.unpack(format!("{}/{}", dest, file.path().unwrap().display()))
                    .unwrap();
            }
        }
        "gz" => {
            let file = File::open(source)?;
            let tar = GzDecoder::new(file);
            let mut archive = Archive::new(tar);

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                file.unpack(format!("{}/{}", dest, file.path().unwrap().display()))
                    .unwrap();
            }
        }
        "xz" => {
            let file = File::open(source)?;
            let tar = XzDecoder::new(file);
            let mut archive = Archive::new(tar);

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                file.unpack(format!("{}/{}", dest, file.path().unwrap().display()))
                    .unwrap();
            }
        }
        "zip" => extract_zip(file_path.to_str().unwrap(), dest).unwrap(),
        "zst" => {
            let mut decoder = {
                let file = File::open(&file_path)?;
                Decoder::new(file)?
            };
            let tar_path = source.to_string().trim_end_matches(".zst").to_string();
            let mut target = File::create(&tar_path)?;
            copy(&mut decoder, &mut target)?;
            extract_archive(&tar_path, dest)?;
        }
        _ => {}
    }

    Ok(())
}

pub fn decompress_zstd(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        Decoder::new(file)?
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX_APP.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}
