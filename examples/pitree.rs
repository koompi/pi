use serde::{Deserialize, Serialize};
use serde_yaml;
use solvent::DepGraph;
use std::{fs::File, io::Read};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub deps: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Store {
    apps: Vec<App>,
}

fn main() {
    let file = File::open("data.yml").unwrap();
    let data: Store = serde_yaml::from_reader(file).unwrap();

    let mut depgraph: DepGraph<String> = DepGraph::new();

    for app in data.apps.iter() {
        if let Some(data) = &app.deps {
            depgraph.register_dependencies(app.name.clone(), data.to_vec());
        } else {
            depgraph.register_node(app.name.clone())
        }
    }
    let mut deps: Vec<String> = Vec::new();
    for node in depgraph
        .dependencies_of(&String::from("base-devel"))
        .unwrap()
    {
        deps.push(node.unwrap().to_string())
    }

    // deps.sort();

    println!("{:#?}", deps);
    // println!("{:#?}", depgraph)
}
