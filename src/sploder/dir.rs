use anyhow::{Error, Result};

use crate::cli::opts::Opts;

pub fn process(_in_path: String, _out_path: String, _cli: Opts) -> Result<(), Error> {
    Ok(())
}
