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

    pub fn validate(&self) -> Result<(), Error> {
        let out_path = Path::new(&self.output);
        if self.is_dir() {
            if !out_path.is_dir() {
                return Err(anyhow!("When the supplied input is a directory, '--output' must be provided and must point to an existing directory."));
            } else {
                return Ok(());
            }
        }
        Ok(())
    }
}
