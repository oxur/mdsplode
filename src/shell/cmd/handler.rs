use anyhow::{anyhow, Error, Result};
use clap::parser::ArgMatches;

use crate as mdsplode;
use crate::json::{jq, print};
use crate::md::converter;
use crate::shell::state::State;
use crate::shell::writer;
use crate::sploder::{file, parser};

const PREFIX: &str = "  ";

pub fn banner(mut state: State, _matches: &ArgMatches) -> Result<State, String> {
    state.show_banner = true;
    writer::msg(state, "")
}

pub fn echo(state: State, matches: &ArgMatches) -> Result<State, String> {
    let msg = concat_args(matches, "args");
    writer::msg(state, &msg)
}

pub fn history(state: State, _matches: &ArgMatches) -> Result<State, String> {
    let mut hist = state.history.clone();
    let rev_hist = hist.make_contiguous();
    rev_hist.reverse();
    writer::msg(state.clone(), format_list(rev_hist.into()).as_str())
}

pub fn ping(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, format_str("pong").as_str())
}

pub fn query(state: State, matches: &ArgMatches) -> Result<State, String> {
    if state.parsed.is_empty() {
        return Err("Parsed is empty, cannot run query. Has a file been read?".to_string());
    };
    let query = concat_args(matches, "query-string");
    match jq::query(state.clone().parsed, query) {
        Ok(r) => match matches.get_one::<bool>("pretty-print") {
            Some(true) => match print::pretty(r) {
                Ok(pp) => writer::msg(state.clone(), pp.as_str()),
                Err(e) => Err(e.to_string()),
            },
            _ => writer::msg(state.clone(), r.as_str()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub fn quit(mut state: State, _matches: &ArgMatches) -> Result<State, String> {
    state.quit = true;
    writer::msg(state, format_str("Quitting ...").as_str())
}

pub fn read(mut state: State, matches: &ArgMatches) -> Result<State, String> {
    match matches.subcommand() {
        Some(("md", md_matches)) => {
            log::debug!("Reading Markdown file ...");
            let filename = match_filename(md_matches);
            state.in_file = filename.clone();
            match file::read_to_string(filename.clone()) {
                Ok(s) => {
                    state.source = s.clone();
                    state.parsed = parser::parse_node(&converter::string_to_mdast(s)).to_json();
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }?;
            writer::msg(
                state,
                format_string(format!("Loaded \"{}\"", filename)).as_str(),
            )
        }
        Some(("json", json_matches)) => {
            log::debug!("Reading JSON file ...");
            let filename = match_filename(json_matches);
            state.in_file = filename.clone();
            match file::read_to_string(filename.clone()) {
                Ok(s) => {
                    state.source = s.clone();
                    state.parsed = s;
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            }?;
            writer::msg(
                state,
                format_string(format!("Loaded \"{}\"", filename)).as_str(),
            )
        }
        Some((name, _)) => unimplemented!("{name}"),
        None => writer::msg(state, format_str("No read subcommand matched").as_str()),
    }
}

pub fn show(state: State, matches: &ArgMatches) -> Result<State, String> {
    match matches.subcommand() {
        Some(("frontmatter", _)) => match frontmatter(state.clone().parsed) {
            Ok(r) => writer::msg(state.clone(), r.as_str()),
            Err(e) => Err(e.to_string()),
        },
        Some(("in-file", _)) => writer::msg(state.clone(), format_string(state.in_file).as_str()),
        Some(("parsed", _)) => writer::msg(state.clone(), format_string(state.parsed).as_str()),
        Some(("source", _)) => writer::msg(state.clone(), format_string(state.source).as_str()),
        Some((name, _)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}
pub fn version(state: State, _matches: &ArgMatches) -> Result<State, String> {
    writer::msg(state, format_str(mdsplode::VERSION).as_str())
}

// Private functions

fn format_str(item: &str) -> String {
    format!("\n{}{}\n", PREFIX, item.trim())
}

fn format_string(item: String) -> String {
    format!("\n{}{}\n", PREFIX, item.trim())
}

fn format_list(list: Vec<String>) -> String {
    let mut res: Vec<String> = vec![PREFIX.to_string()];
    res.append(&mut list.iter().map(|x| format!("{}\n", x.trim())).collect());
    format!("\n{}{}\n", PREFIX, res.join(PREFIX).trim())
}

fn concat_args(matches: &ArgMatches, id: &str) -> String {
    matches
        .get_many::<String>(id)
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn match_filename(matches: &ArgMatches) -> String {
    matches.get_one::<String>("filename").unwrap().to_string()
}

fn frontmatter(parsed: String) -> Result<String, Error> {
    let q = ".children.nodes[] | select(.name == \"toml\") | .json";
    let j_string = jq::query(parsed, q.to_string())?;
    match serde_json::from_str(j_string.as_str()) {
        Ok(r) => print::pretty(r),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}
