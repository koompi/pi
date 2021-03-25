use crate::statics::{LOCAL_DIR, ROOT_DIR};
use crate::Application;
use std::error::Error;
use std::fs::{remove_dir_all, remove_file};
use std::path::PathBuf;

pub fn remove_one(app_name: &String) -> Result<(), Box<dyn Error>> {
    let app_dir: PathBuf = LOCAL_DIR.to_owned().join(app_name);
    let app_path: PathBuf = app_dir.join("manifest.yml");
    let app_data = Manifest::from_file(app_path).unwrap();
    let files = app_data.files.clone();

    files.iter().for_each(|file| {
        let f: Vec<String> = file.split(" ").into_iter().map(|a| a.to_string()).collect();
        let fi = f[0].clone();
        remove_file(ROOT_DIR.join(&fi)).unwrap()
    });

    remove_dir_all(app_dir).unwrap();

    Ok(())
}
