use std::io::{self, BufRead, Write};

use anyhow::Result;

use super::state::State;
use super::writer;

pub const HEADLESS_TERM_CHAR: u8 = 0x03;

pub fn line(state: State) -> Result<String, String> {
    write!(io::stdout(), "{}", state.prompt.as_str()).map_err(|e| e.to_string())?;
    writer::flush()?;
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if state.with_ext_line_ending {
        let mut headless_buffer = vec![];
        handle
            .read_until(HEADLESS_TERM_CHAR, &mut headless_buffer)
            .map_err(|e| e.to_string())?;
        buffer = headless_buffer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
    } else {
        handle.read_line(&mut buffer).map_err(|e| e.to_string())?;
    }
    Ok(buffer)
}
