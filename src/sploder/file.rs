use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Error, Result};

use crate::cli::opts::Opts;
use crate::parse_file;

use super::util::{pretty_print, run_query};

pub fn process(in_file: String, _out_file: String, cli: Opts) -> Result<(), Error> {
    // --input - already taken care of in validation
    let tree = parse_file(in_file.as_str());
    let mut json = tree.to_json();
    // --query
    if let Some(ref query) = cli.query {
        if let Ok(result) = run_query(json.clone(), query.to_string()) {
            json = result;
        };
    };
    // --pretty
    if cli.pretty {
        json = pretty_print(json)?
    }
    // --output
    if cli.is_stdout() {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        match writeln!(stdout, "{:}", json) {
            Err(e) => Err(anyhow!("{:}", e)),
            _ => Ok(()),
        }
    } else {
        // TODO: create a file for writing, write to it
        unimplemented!()
    }
}
