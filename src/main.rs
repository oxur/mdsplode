use std::env;

use anyhow::{Error, Result};
use clap::Parser;

use mdsplode::cli::opts::Opts;
use mdsplode::cli::reset_sigpipe;
use mdsplode::logging;
use mdsplode::sploder::{dir, file};

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let cli = Opts::parse();
    logging::setup(cli.log_level.clone())?;
    cli.validate()?; // This ensures that when is_dir, input & output are dirs
    let input = cli.input.clone();
    let output = cli.output.clone();
    let cwd = env::current_dir()?;
    log::debug!(
        "Input: {}\nOutput: {}\nIs dir? {}\nCurrent dir: {}",
        input.clone(),
        output.clone(),
        cli.is_dir(),
        cwd.display()
    );
    if cli.is_dir() {
        dir::process(input, output, cli.clone())
    } else {
        file::process(input, output, cli)
    }
}
