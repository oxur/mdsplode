pub mod cli;
pub mod json;
pub mod logging;
pub mod md;
pub mod shell;
pub mod sploder;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn version() -> versions::SemVer {
    versions::SemVer::new(VERSION).unwrap()
}
