use std::io;

use serde::{Deserialize, Serialize};

use crate::cli::{self, opts::Format};

use super::state::State;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Output {
    command: String,
    result: String,
    errors: Vec<String>,
}

pub fn msg(state: State, msg: &str) -> Result<State, String> {
    let out = Output {
        command: match state.history.front() {
            None => "".to_string(),
            Some(s) => s.trim().to_string(),
        },
        result: msg.trim().to_string(),
        ..Default::default()
    };
    log::debug!("Writing to device '{:}' ...", state.device.as_str());
    match state.format {
        Format::Text => raw_msg(state.device.as_str(), msg),
        Format::JSON => {
            let json = match serde_json::to_string(&out) {
                Ok(s) => s,
                Err(e) => e.to_string(),
            };
            raw_msg(state.device.as_str(), json.as_str())
        }
    }?;
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
