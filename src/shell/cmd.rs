use anyhow::Result;
use clap::{value_parser, Arg, Command};

use super::writer;

pub fn dispatch(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("error: Invalid quoting")?;
    let matches = cmd()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("echo", matches)) => {
            let msg = matches
                .get_many::<String>("args")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap_or_default()
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .join(" ");
            writer::msg(&msg, false)
        }
        Some(("ping", _matches)) => writer::msg("Pong", false),
        Some(("quit", _matches)) => writer::msg("Quitting ...", true),
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }
}

fn cmd() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
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
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("echo")
                .about("Respond with passed message")
                .help_template(USAGE_TEMPLATE)
                .arg(
                    Arg::new("args")
                        .num_args(0..)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("ping")
                .about("Get a response")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the shell")
                .help_template(USAGE_TEMPLATE),
        )
}
