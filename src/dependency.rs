use crate::Application;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dependency {
    pub build_dependencies: Option<Vec<String>>,
    pub opt_dependencies: Option<Vec<String>>,
    pub run_dependencies: Option<Vec<String>>,
    pub test_dependencies: Option<Vec<String>>,
}

impl Dependency {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_build_dependencies(&self) -> Result<(), Vec<String>> {
        let mut missing_deps: Vec<String> = Vec::new();
        let _missing_from_bin_db: Vec<String> = Vec::new();
        let _missing_from_source_db: Vec<String> = Vec::new();
        let _missing_from_given_args: Vec<String> = Vec::new();

        match &self.build_dependencies {
            Some(deps) => {
                if !deps.is_empty() {
                    // [x] is each dep installed or not?
                    for dep in deps.iter() {
                        match Application::is_installed(&dep) {
                            Some(_) => {}
                            None => missing_deps.push(dep.to_string()),
                        }
                    }
                    // if not, check in bin repo if the package available to install
                    // if not check if it was provided in args or not
                    if !missing_deps.is_empty() {
                        Ok(())
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    pub fn check_opt_dependencies(&self) -> Result<(), Vec<String>> {
        let _missing_deps: Vec<String> = Vec::new();

        match &self.opt_dependencies {
            Some(deps) => {
                if !deps.is_empty() {
                    // is each dep installed or not?
                    // if not, check in bin repo if the package available to install
                    Ok(())
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    pub fn check_run_dependencies(&self) -> Result<(), Vec<String>> {
        let _missing_deps: Vec<String> = Vec::new();

        match &self.build_dependencies {
            Some(deps) => {
                if !deps.is_empty() {
                    // is each dep installed or not?
                    // if not, check in bin repo if the package available to install
                    // if not check if it was provided in args or not
                    Ok(())
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }

    pub fn check_test_dependencies(&self) -> Result<(), Vec<String>> {
        let _missing_deps: Vec<String> = Vec::new();

        match &self.build_dependencies {
            Some(deps) => {
                if !deps.is_empty() {
                    // is each dep installed or not?
                    // if not, check in bin repo if the package available to install
                    // if not check if it was provided in args or not
                    Ok(())
                } else {
                    Ok(())
                }
            }
            None => Ok(()),
        }
    }
}
