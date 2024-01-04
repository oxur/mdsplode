use std::env;

use anyhow::{Error, Result};
use clap::Parser;

use mdsplode::cli::opts::{Commands, Opts};
use mdsplode::cli::reset_sigpipe;
use mdsplode::sploder::{dir, file};
use mdsplode::{logging, shell};

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let mut cli = Opts::parse();
    cli.post_process();
    logging::setup(cli.clone())?;
    match &cli.command {
        Some(Commands::Shell) => shell::run(cli),
        None => {
            cli.validate()?; // This ensures that when is_dir, input & output are dirs
            let input = cli.input.clone().unwrap();
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
    }
}
