pub mod cmd;
pub mod reader;
pub mod state;
pub mod writer;

use anyhow::{Error, Result};

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
            writer::msg(state.clone(), BANNER)?;
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

const BANNER: &str = r#"
Welcome to

      ,--.!,
   __/   -*-     _____ ____  _       ___   ___      ___
 ,d08b.  '|`    / ___/|    \| T     /   \ |   \    /  _]
 0088MM        (   \_ |  o  ) |    Y     Y|    \  /  [_
 `9MMP'      __ \__  T|   _/| l___ |  O  ||  D  YY    _]
.--------.--|  |/  \ ||  |  |     T|     ||     ||   [_
|        |  _  |\    ||  |  |     |l     !|     ||     T
|__|__|__|_____| \___jl__j  l_____j \___/ l_____jl_____j

To see a list of available commands, type "help" at the prompt.
"#;
