use serde::Serialize;
use serde_yaml;
use std::{fs::File, io::Read};

#[derive(Clone, Debug, Default, Serialize)]
pub struct App {
    pub name: String,
    pub deps: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Store {
    apps: Vec<App>,
}

fn main() {
    let mut buf = String::new();
    let mut file = File::open("data.txt").unwrap();
    file.read_to_string(&mut buf).unwrap();

    let mut store: Store = Store::default();

    for line in buf.lines() {
        let sline = line.to_string();

        let splitted_lines: Vec<String> = sline.split("::").map(|l| l.to_string()).collect();

        if splitted_lines.len() == 1 {
            store.apps.push(App {
                name: splitted_lines[0].to_string(),
                deps: None,
            });
        } else {
            let name: String = splitted_lines[0].to_string();
            let mut deps: Vec<String> = splitted_lines[1]
                .to_string()
                .split(":")
                .map(|d| d.to_string())
                .collect();
            if deps.is_empty() {
                store.apps.push(App { name, deps: None });
            } else {
                deps.dedup();
                store.apps.push(App {
                    name,
                    deps: Some(deps),
                });
            }
        }
    }
    store
        .apps
        .dedup_by(|a, b| a.name.eq_ignore_ascii_case(&b.name));
    let mut res = File::create("data.yml").unwrap();
    serde_yaml::to_writer(res, &store).unwrap();
}
