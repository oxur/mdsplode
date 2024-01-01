pub mod handler;

use anyhow::Result;
use clap::{value_parser, Arg, Command};

use super::state::State;

pub fn dispatch(state: State, line: &str) -> Result<State, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cmd()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("banner", matches)) => handler::banner(state.clone(), matches),
        Some(("echo", matches)) => handler::echo(state.clone(), matches),
        Some(("ping", matches)) => handler::ping(state.clone(), matches),
        Some(("quit", matches)) => handler::quit(state.clone(), matches),
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}

fn cmd() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\n\
        {all-args}
    ";
    // strip out name/version
    const USAGE_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\n\
    ";

    Command::new("shell")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("COMMAND")
        .subcommand_help_heading("COMMMANDS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("banner")
                .about("Show the mdsplode banner")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("echo")
                .about("Respond with the passed message")
                .help_template(USAGE_TEMPLATE)
                .arg(
                    Arg::new("args")
                        .num_args(0..)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("ping")
                .about("Check for liveliness")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the shell")
                .help_template(USAGE_TEMPLATE),
        )
}
