pub mod handler;
pub mod opts;

use anyhow::Result;
pub use opts::opts as command_opts;

use super::state::State;

pub fn dispatch(state: State, line: &str) -> Result<State, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = command_opts()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("banner", matches)) => handler::banner(state.clone(), matches),
        Some(("echo", matches)) => handler::echo(state.clone(), matches),
        Some(("history", matches)) => handler::history(state.clone(), matches),
        Some(("ping", matches)) => handler::ping(state.clone(), matches),
        Some(("query", matches)) => handler::query(state.clone(), matches),
        Some(("quit", matches)) => handler::quit(state.clone(), matches),
        Some(("read", matches)) => handler::read(state.clone(), matches),
        Some(("show", matches)) => handler::show(state.clone(), matches),
        Some(("version", matches)) => handler::version(state.clone(), matches),
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}
