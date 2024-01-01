use clap::parser::ArgMatches;

use crate as mdsplode;
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

pub fn history(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state.clone(), format_list(state.history).as_str())
}

pub fn ping(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, "pong")
}

pub fn query(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, "TBD")
}

pub fn quit(mut state: State, _matches: &ArgMatches) -> Result<State, String> {
    state.quit = true;
    writer::msg(state, "\nQuitting ...\n")
}

pub fn read(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, "TBD")
}

pub fn version(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, mdsplode::version().to_string().as_str())
}

// Private functions

fn format_list(mut list: Vec<String>) -> String {
    const PREFIX: &str = "  ";
    let mut res: Vec<String> = vec![PREFIX.to_string()];
    res.append(&mut list);
    format!("\n{}{}\n", PREFIX, res.join(PREFIX).trim())
}
