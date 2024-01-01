use std::io::{self, Write};

use anyhow::Result;

use super::writer;

pub fn line() -> Result<String, String> {
    write!(io::stdout(), "$ ").map_err(|e| e.to_string())?;
    writer::flush()?;
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
