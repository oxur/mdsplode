pub mod cli;
pub mod logging;
pub mod md;
pub mod shell;
pub mod sploder;

pub use sploder::parser::parse_file;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
