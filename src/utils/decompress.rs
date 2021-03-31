use crate::utils::{extract_archive, extract_zip};
use bzip2::read::BzDecoder;
use colored::Colorize;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::*,
    path::PathBuf,
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

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(100);
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template("{spinner:.green.bold} Extracting: {wide_msg}"),
            );

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                let new_file_path = dest_path.join(&file.path().unwrap());
                let new_dir_path = new_file_path.clone();

                if !new_dir_path.parent().unwrap().exists() {
                    create_dir_all(new_dir_path.parent().unwrap()).unwrap();
                }
                file.set_unpack_xattrs(true);
                match file.unpack(new_file_path) {
                    Ok(_) => pb.set_message(&format!("{:?}", &file.path().unwrap())),
                    Err(e) => println!("{}", &e.to_string().red()),
                }
            }
            pb.finish();
        }
        "gz" => {
            // let file = File::open(source)?;
            let mut option = OpenOptions::new();
            let file = option.read(true).write(false).open(source)?;
            let tar = GzDecoder::new(file);
            let mut archive = Archive::new(tar);

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(100);
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template("{spinner:.green.bold} Extracting: {wide_msg}"),
            );

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                let new_file_path = dest_path.join(&file.path().unwrap());
                let new_dir_path = new_file_path.clone();

                if !new_dir_path.parent().unwrap().exists() {
                    create_dir_all(new_dir_path.parent().unwrap()).unwrap();
                }
                file.set_unpack_xattrs(true);
                match file.unpack(new_file_path) {
                    Ok(_) => pb.set_message(&format!("{:?}", &file.path().unwrap())),
                    Err(e) => println!("{}", &e.to_string().red()),
                }
            }
            pb.finish();
        }
        "xz" => {
            let file = File::open(source)?;
            let tar = XzDecoder::new(file);
            let mut archive = Archive::new(tar);

            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(100);
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template("{spinner:.green.bold} Extracting: {wide_msg}"),
            );

            for file in archive.entries().unwrap() {
                let mut file = file.unwrap();

                let new_file_path = dest_path.join(&file.path().unwrap());
                let new_dir_path = new_file_path.clone();

                if !new_dir_path.parent().unwrap().exists() {
                    create_dir_all(new_dir_path.parent().unwrap()).unwrap();
                }
                file.set_unpack_xattrs(true);
                match file.unpack(new_file_path) {
                    Ok(_) => pb.set_message(&format!("{:?}", &file.path().unwrap())),
                    Err(e) => println!("{}", &e.to_string().red()),
                }
            }
            pb.finish();
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
        let file = File::open(&source)?;
        Decoder::new(file)?
    };
    let tar_path = source.to_string().trim_end_matches(".app").to_string();
    let mut target = File::create(&tar_path)?;
    copy(&mut decoder, &mut target)?;
    Ok(())
}
