use pi::{BuildFile, SourceRepo};
use serde_yaml::{from_reader, to_writer};
use std::time::SystemTime;
use std::{
    env,
    fs::{copy, create_dir_all, File},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    result::Result,
};

fn main() {
    let args: Vec<String> = env::args_os()
        .map(|a| a.to_str().unwrap().to_string())
        .collect();

    let verb = &args[1];
    let repo: Option<&String> = args.get(2);

    let packages: Option<Vec<String>> = if &args.len() >= &(3 as usize) {
        Some(args[3..].to_vec())
    } else {
        None
    };

    match verb.as_ref() {
        "a" | "add" | "-a" | "--add" => {
            if let Some(rep) = repo {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    // pkgs.iter().for_each(|p| println!("{}", p))
                    add(rep, ps)
                } else {
                    println!("No packages was given");
                }
            } else {
                eprintln!("Repo name is require")
            }
        }
        "c" | "create" | "-c" | "--create" => {
            if let Some(rep) = repo {
                create(rep).unwrap();
            } else {
                eprintln!("Repo name is require")
            }
        }
        "r" | "remove" | "-r" | "--remove" => {
            if let Some(rep) = repo {
                if let Some(pkgs) = packages {
                    let ps: Vec<PathBuf> = pkgs.iter().map(|p| PathBuf::from(p)).collect();
                    remove(rep, ps)
                } else {
                    println!("No packages was given");
                }
            } else {
                eprintln!("Repo name is require")
            }
        }
        _ => help(),
    }
}

fn create(path: &str) -> Result<(), Error> {
    let p = Path::new(path);
    if let Some(p) = p.parent() {
        if !p.exists() {
            create_dir_all(p).unwrap();
        }
    }

    let file = File::create(path)?;
    let data = SourceRepo::new();
    match to_writer(file, &data) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn update_db(path: &str, data: &SourceRepo) -> Result<(), Error> {
    let p = Path::new(path);
    if let Some(p) = p.parent() {
        if !p.exists() {
            create_dir_all(p).unwrap();
        }
    }

    let file = File::create(path)?;
    // let data = SourceRepo::default();
    match to_writer(file, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn opendb(path: &str) -> Result<SourceRepo, Error> {
    let db_file = File::open(path)?;
    match from_reader(db_file) {
        Ok(db) => Ok(db),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}

fn add(db_path: &str, pkg_files: Vec<PathBuf>) {
    let mut db: SourceRepo = opendb(db_path).unwrap();

    for pkg_file in pkg_files.iter() {
        let pkg_file_name = pkg_file.to_str().unwrap();

        let file = File::open(pkg_file_name).unwrap();
        let data: BuildFile = from_reader(file).unwrap();

        let db_dir = PathBuf::from(db_path);
        let new_file_name = db_dir
            .parent()
            .unwrap()
            .join(format!("{}.yml", data.metadata.name));

        copy(pkg_file_name, new_file_name).unwrap();

        db.applications
            .entry(data.metadata.name.clone())
            .or_insert(data);
    }
    let now = SystemTime::now();
    db.date = now;
    update_db(db_path, &db).unwrap();
}

fn remove(db_path: &str, pkg_files: Vec<PathBuf>) {
    let mut db: SourceRepo = opendb(db_path).unwrap();

    let db_file = PathBuf::from(db_path);
    let db_dir = db_file.parent().unwrap();

    if !pkg_files.is_empty() {
        for pkg in pkg_files.iter() {
            if let Some((_, app)) = db.applications.remove_entry(pkg.to_str().unwrap()) {
                let file_name = format!("{}.yml", app.metadata.name);
                std::fs::remove_file(db_dir.join(file_name)).unwrap();
            }
        }
    }
    let now = SystemTime::now();
    db.date = now;
    update_db(db_path, &db).unwrap();
}

fn help() {
    print!(
        r#"
USAGE:
source-repo <operation> <repo_name> [packages]

Operations:
    create <repo_name>              generation an empty repo with the given name.
    add <repo_name> [packages]      add the packages to that repo.
    remove <repo_name> [package]    remove the packages to that repo.
"#
    );
}
