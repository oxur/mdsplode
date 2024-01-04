use anyhow::{Error, Result};

use crate::cli::opts::Opts;

use super::state::State;
use super::{cmd, reader, writer};

pub fn run(opts: Opts) -> Result<(), Error> {
    log::debug!("Starting shell ...");
    _ = shell(State::new(opts));
    Ok(())
}

fn shell(mut state: State) -> Result<State, String> {
    loop {
        log::debug!("Got state: {:?}", state);
        if state.show_banner {
            state.show_banner = false;
            let mut colours = Colours::new();
            if !state.without_colour {
                colours = Colours {
                    red: "\x1B[1;31m",
                    yellow_bold: "\x1B[1;33m",
                    yellow: "\x1B[33m",
                    green: "\x1B[32m",
                    blue: "\x1B[34m",
                    white: "\x1B[1;37m",
                    end: "\x1B[0m",
                };
            }
            writer::msg(state.clone(), banner(colours).as_str())?;
            continue;
        }
        let line = reader::line(state.clone())?;
        state.history.push_front(line.clone());
        state.history.truncate(state.history_size);
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match cmd::dispatch(state.clone(), line) {
            Ok(new_state) => {
                state = new_state;
                if state.quit {
                    break;
                }
            }
            Err(err) => {
                writer::msg(state.clone(), err.as_str())?;
            }
        }
    }
    Ok(state)
}

#[derive(Clone, Debug, Default)]
struct Colours {
    red: &'static str,
    yellow_bold: &'static str,
    yellow: &'static str,
    green: &'static str,
    blue: &'static str,
    white: &'static str,
    end: &'static str,
}

impl Colours {
    pub fn new() -> Colours {
        Colours {
            ..Default::default()
        }
    }
}

fn banner(cols: Colours) -> String {
    format!(
        r#"
Welcome to

        {white},--{end}{red}.{end}{yellow}!{end}{red},{end}
     {blue}__{end}{white}/{end}   {yellow}-{end}{yellow_bold}*{end}{yellow}-{end}   {green}_____ ____  _       ___   ___      ___{end}
   {blue},d08b.{end}  {red}'{end}{yellow}|{end}{red}`{end}  {green}/ ___/|    \| T     /   \ |   \    /  _]{end}
   {blue}0088MM{end}      {green}(   \_ |  P  ) |    Y     Y|    \  /  [_{end}
   {blue}`9MMP'{end}    __ {green}\__  T|   _/| l___ |  O  ||  D  YY    _]
{green}.--------.--|  |/  \ ||  |  |     T|     ||     ||   [_
|        |  _  |\    ||  |  |     |l     !|     ||     T
|__|__|__|_____| \___jl__j  l_____j \___/ l_____jl_____j{end}

v{version}

To see a list of available commands, type "help" at the prompt.
"#,
        red = cols.red,
        yellow_bold = cols.yellow_bold,
        yellow = cols.yellow,
        green = cols.green,
        blue = cols.blue,
        white = cols.white,
        end = cols.end,
        version = crate::VERSION,
    )
}
