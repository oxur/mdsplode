use std::io::{self, Write};

use anyhow::Result;

use super::state::State;
use super::writer;

pub fn line(state: State) -> Result<String, String> {
    write!(io::stdout(), "{}", state.prompt.as_str()).map_err(|e| e.to_string())?;
    writer::flush()?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
