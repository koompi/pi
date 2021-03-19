use super::{Application};
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_yaml;
use solvent::DepGraph;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Database {
    applications: Vec<Application>
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let data: Database = serde_yaml::from_reader(file).unwrap();
        data
    }

    pub fn initialize_depgraph(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for app in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.run_dependencies {
                        let rdeps = run_deps;
                        depgraph.register_dependencies(name, rdeps.to_vec())
                    }
                } else {
                    depgraph.register_node(name)
                }
            }
        }
        depgraph
    }
}
