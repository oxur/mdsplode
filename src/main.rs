use std::io::{self, Write};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use jsonpath::Selector;
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
    #[arg(short, long, help = "Optionally filter on a JSONpath selector")]
    selector: Option<String>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let tree = mdsplode::parse_file(cli.input.as_str());
    let mut json = tree.to_json();
    match cli.selector {
        Some(sel) => {
            // println!("Got selector: {:?}", sel);
            let data: Value = serde_json::from_str(&json)?;
            match Selector::new(&sel) {
                Ok(s) => {
                    json = s
                        .find(&data)
                        .map(|t| match t.as_str() {
                            Some(str) => str,
                            _ => "",
                        })
                        .collect();
                    // println!("JSON: {:}", json);
                }
                Err(e) => {
                    return Err(anyhow!(
                        "{:}. Couldn't create selector from passed value.",
                        e
                    ));
                }
            }
        }
        _ => (),
    };
    match cli.output.as_str() {
        STDOUT => println!("{:}", json),
        _ => unimplemented!(),
    };
    Ok(())
}
