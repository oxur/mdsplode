use std::io::prelude::*;
use std::{fs, io};

use anyhow::{anyhow, Error, Result};

use crate::cli::opts::Opts;
use crate::parse_file;

use super::util::{pretty_print, run_query};

pub fn process(in_file: String, out_file: String, cli: Opts) -> Result<(), Error> {
    log::info!("Processing file: {}", in_file);
    let mut json: String;
    // --skip-process
    if cli.skip_process {
        json = read_to_string(in_file)?;
    } else {
        // --input - already taken care of in validation
        let tree = parse_file(in_file.as_str());
        json = tree.to_json();
    }
    // --query
    if let Some(ref query) = cli.query {
        if let Ok(result) = run_query(json.clone(), query.to_string()) {
            json = result;
        };
    };
    // --pretty
    if cli.pretty {
        json = pretty_print(json.clone())?
    }
    // --output
    if cli.is_stdout() {
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        match writeln!(stdout, "{}", json.clone()) {
            Err(e) => Err(anyhow!("{:}", e)),
            _ => Ok(()),
        }
    } else {
        let mut out = fs::File::create(out_file)?;
        match write!(out, "{}", json) {
            Err(e) => Err(anyhow!("{:}", e)),
            _ => Ok(()),
        }
    }
}

fn read_to_string(filename: String) -> Result<String, Error> {
    match fs::read_to_string(filename.as_str()) {
        Err(e) => Err(anyhow!("{:}", e)),
        Ok(data) => Ok(data),
    }
}
