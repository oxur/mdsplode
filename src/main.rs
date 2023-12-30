use std::io;
use std::io::prelude::*;

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use serde_json::Value;

const STDOUT: &str = "stdout";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true, help = "Input file or directory")]
    input: String,
    #[arg(
        short,
        long,
        default_value = STDOUT,
        help = "Output file or directory"
    )]
    output: String,
    #[arg(long, short, action)]
    pretty: bool,
    #[arg(short, long, help = "Optionally filter with a jq query string")]
    query: Option<String>,
}

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let cli = Cli::parse();
    let tree = mdsplode::parse_file(cli.input.as_str());
    let mut json = tree.to_json();
    if let Some(query) = cli.query {
        if let Ok(result) = jq_rs::run(query.as_str(), &json) {
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

fn pretty_print(json: String) -> Result<String, Error> {
    match serde_json::from_str::<Value>(json.as_str()) {
        Ok(obj) => match serde_json::to_string_pretty(&obj) {
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow!(
                "Could not convert json fragment to pretty-printed string: {}",
                e
            )),
        },
        Err(e) => Err(anyhow!(
            "Could not convert json string back to object for pretty-printing: {}",
            e
        )),
    }
}

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {
    // no-op
}
