pub mod cmd;
pub mod reader;
pub mod writer;

use anyhow::{Error, Result};

use crate::cli::opts::Opts;

pub fn run(opts: Opts) -> Result<(), Error> {
    log::debug!("Starting shell ...");
    _ = shell(opts);
    Ok(())
}

fn shell(_opts: Opts) -> Result<(), String> {
    loop {
        let line = reader::line()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match cmd::dispatch(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                writer::msg(err.as_str(), false)?;
            }
        }
    }
    Ok(())
}
