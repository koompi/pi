use std::{
    fs::{metadata, File},
    io::prelude::*,
    path::Path,
};

pub fn read_to_vec_u8(filename: &Path) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
