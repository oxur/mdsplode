use std::path::Path;

use anyhow::{anyhow, Error, Result};
use clap::{ArgGroup, Parser, Subcommand};

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
        action,
        help = "Set line delimiter to use ETX (0x03) 'end of text'",
        help_heading = "Shell Options",
        global = true
    )]
    pub etx_line_ending: bool,
    #[arg(
        long,
        action,
        help = "Set if the shell will be read programmatically and not in an interactive session; implies --etx-line-ending, --no-banner, and --no-colour",
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
        if self.etx_line_ending {
            self.etx_line_ending = true;
            self.no_banner = true;
            self.no_colour = true;
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
