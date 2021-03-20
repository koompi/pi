use crate::statics::*;
use std::{
    fs::File,
    io::{copy, Result},
};

pub fn compress_zstd(source: &str) -> Result<()> {
    let mut file = File::open(source)?;
    let mut encoder = {
        let target = File::create(source.to_string() + &SUFFIX_APP)?;
        zstd::Encoder::new(target, 1)?
    };

    copy(&mut file, &mut encoder)?;
    match encoder.finish() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
