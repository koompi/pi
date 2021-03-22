use super::BuildFile;
use serde::{Deserialize, Serialize};
use serde_yaml;
use solvent::DepGraph;
use std::fs::File;
use std::time::SystemTime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourceDatabase {
    pub applications: Vec<BuildFile>,
    pub date: SystemTime,
}

impl SourceDatabase {
    // Create
    pub fn new() -> Self {
        Self {
            applications: Vec::new(),
            date: SystemTime::now(),
        }
    }

    pub fn from(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let data: SourceDatabase = serde_yaml::from_reader(file).unwrap();
        data
    }

    // Dependencies
    pub fn get_run_dependencies(&self) -> DepGraph<String> {
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

    pub fn get_opt_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for app in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.opt_depencies {
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

    pub fn get_build_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for app in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.build_dependencies {
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

    pub fn get_test_dependencies(&self) -> DepGraph<String> {
        let mut depgraph: DepGraph<String> = DepGraph::new();
        if !self.applications.is_empty() {
            for app in self.applications.iter() {
                let name = app.metadata.name.to_string();
                if let Some(deps) = &app.dependencies {
                    if let Some(run_deps) = &deps.test_dependencies {
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
