use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Dependency {
    pub build_depencies: Option<Vec<String>>,
    pub opt_depencies: Option<Vec<String>>,
    pub run_dependencies: Option<Vec<String>>,
    pub test_depencies: Option<Vec<String>>,
}
