pub mod cmd;
pub mod reader;
pub mod state;
pub mod writer;

use anyhow::{Error, Result};
use const_format::formatcp;

use crate::cli::opts::Opts;

use state::State;

pub const DEFAULT_PROMPT: &str = "sploder> ";

pub fn run(opts: Opts) -> Result<(), Error> {
    log::debug!("Starting shell ...");
    _ = shell(State::new(opts));
    Ok(())
}

fn shell(mut state: State) -> Result<State, String> {
    loop {
        log::debug!("Got state: {:?}", state);
        if state.show_banner {
            state.show_banner = false;
            writer::msg(state.clone(), banner())?;
            continue;
        }
        let line = reader::line(state.clone())?;
        state.history.push(line.clone());
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match cmd::dispatch(state.clone(), line) {
            Ok(new_state) => {
                state = new_state;
                if state.quit {
                    break;
                }
            }
            Err(err) => {
                writer::msg(state.clone(), err.as_str())?;
            }
        }
    }
    Ok(state)
}

fn banner() -> &'static str {
    const RED: &str = "\x1B[1;31m";
    const YELLOW_BOLD: &str = "\x1B[1;33m";
    const YELLOW: &str = "\x1B[33m";
    const GREEN: &str = "\x1B[32m";
    const BLUE: &str = "\x1B[34m";
    const WHITE: &str = "\x1B[1;37m";

    const END: &str = "\x1B[0m";
    formatcp!(
        r#"
Welcome to

        {WHITE},--{END}{RED}.{END}{YELLOW}!{END}{RED},{END}
     {BLUE}__{END}{WHITE}/{END}   {YELLOW}-{END}{YELLOW_BOLD}*{END}{YELLOW}-{END}   {GREEN}_____ ____  _       ___   ___      ___{END}
   {BLUE},d08b.{END}  {RED}'{END}{YELLOW}|{END}{RED}`{END}  {GREEN}/ ___/|    \| T     /   \ |   \    /  _]{END}
   {BLUE}0088MM{END}      {GREEN}(   \_ |  o  ) |    Y     Y|    \  /  [_{END}
   {BLUE}`9MMP'{END}    __ {GREEN}\__  T|   _/| l___ |  O  ||  D  YY    _]
{GREEN}.--------.--|  |/  \ ||  |  |     T|     ||     ||   [_
|        |  _  |\    ||  |  |     |l     !|     ||     T
|__|__|__|_____| \___jl__j  l_____j \___/ l_____jl_____j{END}

To see a list of available commands, type "help" at the prompt.
"#
    )
}
