use clap::Parser;

use super::STDOUT;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, required = true, help = "Input file or directory")]
    pub input: String,
    #[arg(
        short,
        long,
        default_value = STDOUT,
        help = "Output file or directory"
    )]
    pub output: String,
    #[arg(long, short, action)]
    pub pretty: bool,
    #[arg(short, long, help = "Optionally filter with a jq query string")]
    pub query: Option<String>,
}
