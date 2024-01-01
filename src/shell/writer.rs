use std::io::{self, Write};

use super::state::State;

pub fn msg(state: State, msg: &str) -> Result<State, String> {
    writeln!(io::stdout(), "{}", msg).map_err(|e| e.to_string())?;
    flush()?;
    Ok(state)
}

pub fn flush() -> Result<(), String> {
    io::stdout().flush().map_err(|e| e.to_string())
}
