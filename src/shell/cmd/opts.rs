use clap::{value_parser, Arg, ArgAction, Command};

pub fn opts() -> Command {
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
            Command::new("history")
                .alias("hist")
                .about("Show all commands entered so far")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("ping")
                .about("Check for liveliness")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("query")
                .alias("q")
                .alias("jq")
                .about("Perform a jq-style query on the most recently read data (aliases: q, jq)")
                .help_template(USAGE_TEMPLATE)
                .arg(
                    Arg::new("pretty-print")
                        .long("pretty")
                    .help(
                        "Pretty-print the JSON result; only usable on queries that produce valid JSON",)
                    .action(ArgAction::SetTrue))
                .arg(
                    Arg::new("query-string")
                        .num_args(0..)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the shell (alias: exit)")
                .help_template(USAGE_TEMPLATE),
        )
        .subcommand(
            Command::new("read")
                .alias("r")
                .about("Read in a data source (alias: r)")
                .help_template(USAGE_TEMPLATE)
                .subcommand(
                    Command::new("md")
                        .about("Set the data source to be read as Markdown")
                        .help_template(USAGE_TEMPLATE)
                        .arg(Arg::new("filename").required(true)),
                )
                .subcommand(
                    Command::new("json")
                        .about("Set the data source to be read as JSON")
                        .help_template(USAGE_TEMPLATE)
                        .arg(Arg::new("filename").required(true)),
                ),
        )
        .subcommand(
            Command::new("show")
                .alias("sh")
                .about("Show various aspects of the mdsplode state (alias: sh)")
                .help_template(USAGE_TEMPLATE)
                .subcommand(
                    Command::new("frontmatter")
                        .alias("fm")
                        .about("Show the parsed front matter data")
                        .help_template(USAGE_TEMPLATE),
                )
                .subcommand(
                    Command::new("in-file")
                        .about("Show the input filename")
                        .help_template(USAGE_TEMPLATE),
                )
                .subcommand(
                    Command::new("parsed")
                        .about("Show the parsed Markdown data (JSON)")
                        .help_template(USAGE_TEMPLATE),
                )
                .subcommand(
                    Command::new("source")
                        .about("Show the Markdown source")
                        .help_template(USAGE_TEMPLATE),
                ),
        )
        .subcommand(
            Command::new("version")
                .alias("ver")
                .about("Display the current version")
                .help_template(USAGE_TEMPLATE),
        )
}
