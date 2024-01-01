pub mod cmd;
pub mod reader;
pub mod state;
pub mod writer;

use anyhow::{Error, Result};

use crate::cli::opts::Opts;

use state::State;

pub fn run(opts: Opts) -> Result<(), Error> {
    log::debug!("Starting shell ...");
    let state = State::new();
    _ = shell(state, opts);
    Ok(())
}

fn shell(state: State, _opts: Opts) -> Result<State, String> {
    loop {
        let line = reader::line()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match cmd::dispatch(state.clone(), line) {
            Ok(new_state) => {
                if new_state.quit {
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
