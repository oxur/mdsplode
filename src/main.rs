use std::io;
use std::io::prelude::*;

use anyhow::{Error, Result};
use clap::Parser;
use jq_rs;

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
    #[arg(short, long, help = "Optionally filter with a jq query string")]
    query: Option<String>,
}

fn main() -> Result<(), Error> {
    reset_sigpipe();
    let cli = Cli::parse();
    let tree = mdsplode::parse_file(cli.input.as_str());
    let mut json = tree.to_json();
    match cli.query {
        Some(query) => {
            match jq_rs::run(query.as_str(), &json) {
                Ok(result) => {
                    json = result;
                }
                _ => (),
            };
        }
        _ => (),
    };
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    match cli.output.as_str() {
        STDOUT => writeln!(stdout, "{:}", json)?,
        _ => unimplemented!(),
    };
    Ok(())
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
