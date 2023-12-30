pub mod md;
pub mod sploder;

pub use sploder::parser::parse_file;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
