use std::path::Path;

use anyhow::{anyhow, Error, Result};
use clap::Parser;

use super::STDOUT;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    #[arg(short, long, required = true, help = "Input file or directory")]
    pub input: String,
    #[arg(
        long,
        short,
        default_value = "error",
        help = "If logging is enabled, log at this level"
    )]
    pub log_level: String,

    #[arg(
        short,
        long,
        default_value = STDOUT,
        help = "Output file or directory"
    )]
    pub output: String,
    #[arg(long, short, action, help = "If provided, pretty-print output JSON")]
    pub pretty: bool,
    #[arg(short, long, help = "Optionally filter output with a jq query string")]
    pub query: Option<String>,
    #[arg(
        long,
        short,
        action,
        help = "If provided, skip the Markdown processing; assume input files are pre-processed mdsplode output JSON data files"
    )]
    pub skip_process: bool,
}

impl Opts {
    pub fn is_dir(&self) -> bool {
        Path::new(&self.input).is_dir()
    }

    pub fn is_file(&self) -> bool {
        Path::new(&self.input).is_file()
    }

    pub fn is_stdout(&self) -> bool {
        self.output.as_str() == STDOUT
    }

    pub fn setup_logging(&self) {}

    pub fn validate(&self) -> Result<(), Error> {
        let out_path = Path::new(&self.output);
        if self.is_dir() {
            if !out_path.is_dir() && !self.skip_process {
                return Err(anyhow!("When the supplied input is a directory, '--output' must be provided and must point to an existing directory."));
            } else {
                return Ok(());
            }
        }
        Ok(())
    }
}
