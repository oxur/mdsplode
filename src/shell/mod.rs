pub mod cmd;
pub mod runner;
pub mod state;
pub mod writer;

pub use runner::run;

pub const DEFAULT_PROMPT: &str = "sploder> ";
pub const DEFAULT_HISTORY_SIZE: usize = 1000;
