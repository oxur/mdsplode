use std::io;

use crate::cli;

use super::state::State;

pub fn msg(state: State, msg: &str) -> Result<State, String> {
    // let d = state.clone().device.ok_or("no device".to_string())?;
    log::debug!("Writing to device '{:}' ...", state.device.as_str());
    raw_msg(state.device.as_str(), msg)?;
    flush_stderr()?;
    flush_stdout()?;
    Ok(state)
}

pub fn flush(state: &State) -> Result<(), String> {
    // let d = state.clone().device.ok_or("no device".to_string())?;
    raw_flush(state.device.as_str())
}

pub fn stderr(msg: &str) -> Result<(), String> {
    raw_msg(cli::STDERR, msg)
}

pub fn stdout(msg: &str) -> Result<(), String> {
    raw_msg(cli::STDOUT, msg)
}

pub fn flush_stdout() -> Result<(), String> {
    raw_flush(cli::STDOUT)
}

pub fn flush_stderr() -> Result<(), String> {
    raw_flush(cli::STDERR)
}

pub fn raw_msg(device: &str, msg: &str) -> Result<(), String> {
    let (mut stdout, mut stderr);
    let stream: &mut dyn io::Write = if device == cli::STDERR {
        stderr = io::stderr();
        &mut stderr
    } else {
        stdout = io::stdout();
        &mut stdout
    };
    writeln!(stream, "{}", msg).map_err(|e| e.to_string())
}

pub fn raw_flush(device: &str) -> Result<(), String> {
    let (mut stdout, mut stderr);
    let stream: &mut dyn io::Write = if device == cli::STDERR {
        stderr = io::stderr();
        &mut stderr
    } else {
        stdout = io::stdout();
        &mut stdout
    };
    stream.flush().map_err(|e| e.to_string())
}
