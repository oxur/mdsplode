use clap::parser::ArgMatches;

use crate::shell::state::State;
use crate::shell::writer;

pub fn banner(mut state: State, _matches: &ArgMatches) -> Result<State, String> {
    state.show_banner = true;
    writer::msg(state, "")
}

pub fn echo(state: State, matches: &ArgMatches) -> Result<State, String> {
    let msg = matches
        .get_many::<String>("args")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    writer::msg(state, &msg)
}

pub fn ping(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, "pong")
}

pub fn quit(mut state: State, _matches: &ArgMatches) -> Result<State, String> {
    state.quit = true;
    writer::msg(state, "\nQuitting ...\n")
}
