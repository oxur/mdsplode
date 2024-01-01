use std::io::{self, Write};

pub fn msg(msg: &str, quit: bool) -> Result<bool, String> {
    writeln!(io::stdout(), "{}", msg).map_err(|e| e.to_string())?;
    flush()?;
    Ok(quit)
}

pub fn flush() -> Result<(), String> {
    io::stdout().flush().map_err(|e| e.to_string())
}
