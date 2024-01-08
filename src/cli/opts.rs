use std::{fmt, path::Path, str::FromStr};

use anyhow::{anyhow, Error, Result};
use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{ArgGroup, Parser, Subcommand};

use crate::cli;
use crate::shell::{DEFAULT_HISTORY_SIZE, DEFAULT_PROMPT};

use super::STDOUT;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(group = ArgGroup::new("shell").multiple(false))]
pub struct Opts {
    #[arg(long, default_value_t = DEFAULT_HISTORY_SIZE, help = "Number of commands to keep in the shell history")]
    pub history_size: usize,
    #[arg(short, long, help = "Input file or directory")]
    pub input: Option<String>,
    #[arg(
        long,
        short,
        default_value = "error",
        help = "If logging is enabled, log at this level",
        global = true
    )]
    pub log_level: String,
    #[arg(
        long,
        action,
        help = "If provided, ANSI colours are not used in output"
    )]
    pub no_colour: bool,
    #[arg(
        short,
        long,
        default_value = STDOUT,
        help = "Output file or directory"
    )]
    pub output: String,
    #[arg(long, action, help = "If provided, pretty-print output JSON")]
    pub pretty: bool,
    #[arg(short, long, help = "Optionally filter output with a jq query string")]
    pub query: Option<String>,
    #[arg(
        long,
        short,
        action,
        help = "If provided, skip the Markdown processing; assume input files are pre-processed mdsplode output JSON data files"
    )]
    pub skip_process: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

    // Shell options
    #[arg(
        long,
        default_value = "stdout",
        help = "The output device to use when writing results",
        help_heading = "Shell Options",
        global = true
    )]
    pub device: Option<String>,
    #[arg(
        long,
        default_value = "stderr",
        help = "The output device to use for writing logging output",
        help_heading = "Shell Options",
        global = true
    )]
    pub log_device: Option<String>,
    #[arg(
        long,
        default_value = "text",
        help = "The format in which to serialise output",
        help_heading = "Shell Options",
        value_parser = PossibleValuesParser::new(["text", "json"])
        .map(|s| s.parse::<Format>().unwrap()),
        global = true
    )]
    pub format: Format,
    #[arg(
        long,
        action,
        help = "Set if the shell will be read programmatically and not in an interactive session; implies '--format json', '--no-banner', '--no-colour', and 'prompt \"\"' (pretty is also hard-coded to 'false')",
        help_heading = "Shell Options",
        global = true
    )]
    pub headless: bool,
    #[arg(
        long,
        action,
        help = "Set to disable printing the banner upon shell startup",
        help_heading = "Shell Options",
        global = true
    )]
    pub no_banner: bool,
    #[arg(
        long,
        default_value = DEFAULT_PROMPT,
        help = "Override the default shell prompt",
        help_heading = "Shell Options",
        global = true
    )]
    pub prompt: String,
}

impl Opts {
    pub fn is_dir(&self) -> bool {
        Path::new(&self.input.clone().unwrap()).is_dir()
    }

    pub fn is_file(&self) -> bool {
        Path::new(&self.input.clone().unwrap()).is_file()
    }

    pub fn is_stdout(&self) -> bool {
        self.output.as_str() == STDOUT
    }

    pub fn post_process(&mut self) {
        if self.device.is_none() {
            self.device = cli::stdout()
        }
        if self.log_device.is_none() {
            self.log_device = cli::stderr()
        }
        if self.headless {
            self.format = Format::JSON;
            self.no_banner = true;
            self.no_colour = true;
            self.pretty = false;
            self.prompt = "".to_string();
        }
    }

    pub fn setup_logging(&self) {}

    // validate is only intended for use with the CLI; current design will break if
    // one attempts to use it with the shell.
    pub fn validate(&self) -> Result<(), Error> {
        match self.input {
            None => Err(anyhow!(
                "When using mdsplode's CLI, the 'input' parameter is required."
            )),
            _ => Ok(()),
        }?;
        let out_path = Path::new(&self.output);
        if self.is_dir() {
            if !out_path.is_dir() && !self.skip_process {
                return Err(anyhow!("When the supplied input is a directory, '--output' must be provided and must point to an existing directory."));
            } else {
                return Ok(());
            }
        }
        Ok(())
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    Shell,
}

#[derive(Clone, Debug, Default)]
pub enum Format {
    #[default]
    Text,
    JSON,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::JSON => write!(f, "json"),
        }
    }
}

impl FromStr for Format {
    type Err = ();

    fn from_str(input: &str) -> Result<Format, Self::Err> {
        match input.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::JSON),
            _ => Err(()),
        }
    }
}
