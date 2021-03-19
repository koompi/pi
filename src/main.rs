#![allow(unused_variables)]
pub mod utils;

mod application;
mod architecture;
mod build_file;
mod database;
mod dependency;
mod deployment;
mod function;
mod license;
mod metadata;
mod security;
mod source;
// Local
pub use application::Application;
pub use architecture::Architecture;
pub use build_file::BuildFile;
pub use dependency::Dependency;
pub use deployment::Deployment;
pub use function::Function;
pub use license::License;
pub use metadata::Metadata;
pub use security::Security;
pub use source::Source;
// External

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
}
