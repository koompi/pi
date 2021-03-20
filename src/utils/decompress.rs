use crate::statics::*;
use crate::utils::{extract_archive, extract_zip};
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{copy, Result},
    path::PathBuf,
    str,
};
use xz2::read::XzDecoder;
use zstd::Decoder;

pub fn decompress_zstd(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        Decoder::new(file)?
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX_APP.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}

pub fn decompress_xz(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        XzDecoder::new(file)
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX_APP.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}

pub fn decompress_bz2(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        BzDecoder::new(file)
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX_APP.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}

pub fn decompress_gz(source: &str) -> Result<()> {
    let mut decoder = {
        let file = File::open(source)?;
        GzDecoder::new(file)
    };

    let mut target = File::create(source.trim_end_matches(SUFFIX_APP.as_str()))?;

    copy(&mut decoder, &mut target)?;

    Ok(())
}

pub fn decompress_all(source: &str, dest: &str) -> Result<()> {
    let file_path: PathBuf = PathBuf::from(source);

    match file_path.extension().unwrap().to_str().unwrap() {
        "bz2" => {
            decompress_bz2(file_path.to_str().unwrap()).unwrap();
            let mut target = source.trim_end_matches(".bz2");
            extract_archive(target, dest);
        }
        "gz" => {
            println!("{}", file_path.display());
            decompress_gz(file_path.to_str().unwrap()).unwrap();
            let mut target = source.trim_end_matches(".gz");
            extract_archive(target, dest);
        }
        "xz" => {
            decompress_xz(file_path.to_str().unwrap()).unwrap();
            let mut target = source.trim_end_matches(".xz");
            extract_archive(target, dest);
        }
        "zip" => extract_zip(file_path.to_str().unwrap(), dest).unwrap(),
        "zst" => {
            decompress_zstd(file_path.to_str().unwrap()).unwrap();
            let mut target = source.trim_end_matches(".zst");
            extract_archive(target, dest);
        }
        _ => {}
    }
    Ok(())
}
