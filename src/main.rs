use std::io;
use std::io::prelude::*;

use anyhow::{Error, Result};
use clap::Parser;

use mdsplode::cli::{opts, pretty_print, reset_sigpipe, run_query, STDOUT};
use mdsplode::parse_file;

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let cli = opts::Cli::parse();
    let tree = parse_file(cli.input.as_str());
    let mut json = tree.to_json();
    if let Some(query) = cli.query {
        if let Ok(result) = run_query(json.clone(), query) {
            json = result;
        };
    };
    if cli.pretty {
        json = pretty_print(json)?
    }
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    match cli.output.as_str() {
        STDOUT => writeln!(stdout, "{:}", json)?,
        _ => unimplemented!(),
    };
    Ok(())
}
