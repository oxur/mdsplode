pub mod md;
pub mod splode;

pub use splode::parse_file;

pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
