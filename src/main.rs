use anyhow::{Error, Result};
use clap::Parser;

use mdsplode::cli::opts::Opts;
use mdsplode::cli::reset_sigpipe;
use mdsplode::sploder::{dir, file};

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let cli = Opts::parse();
    cli.validate()?; // This ensures that when is_dir, input & output are dirs
    let input = cli.input.clone();
    let output = cli.output.clone();
    if cli.is_dir() {
        dir::process(input, output, cli.clone())
    } else {
        file::process(input, output, cli)
    }
}
