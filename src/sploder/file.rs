use std::io::prelude::*;
use std::{fs, io};

use anyhow::{anyhow, Error, Result};
use markdown::mdast::Node;

use crate::cli::opts::Opts;
use crate::json::{jq, print};
use crate::md::converter;

use super::parser;
use super::types::CompoundNode;

pub fn process(in_file: String, out_file: String, cli: Opts) -> Result<(), Error> {
    log::info!("Processing file: {}", in_file);
    let mut json: String;
    // --skip-process
    if cli.skip_process {
        json = read_to_string(in_file)?;
    } else {
        // --input - already taken care of in validation
        let tree = read_to_parsed(in_file)?;
        json = tree.to_json();
    }
    // --query
    if let Some(ref query) = cli.query {
        if let Ok(result) = jq::query(json.clone(), query.to_string()) {
            json = result;
        };
    };
    // --pretty
    if cli.pretty {
        json = print::pretty(json.clone())?
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

pub fn read_to_string(filename: String) -> Result<String, Error> {
    match fs::read_to_string(filename.as_str()) {
        Err(e) => Err(anyhow!("{:}", e)),
        Ok(data) => Ok(data),
    }
}

pub fn read_to_mdast(filename: String) -> Result<Node, Error> {
    let md = read_to_string(filename)?;
    Ok(converter::string_to_mdast(md))
}

pub fn read_to_parsed(filename: String) -> Result<CompoundNode, Error> {
    let node = read_to_mdast(filename)?;
    Ok(parser::parse_node(&node))
}
