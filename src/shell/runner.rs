use std::fs;
use std::path::Path;

use anyhow::{Error, Result};
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::cli::opts::Opts;

use super::state::State;
use super::{cmd, writer};

pub fn run(opts: Opts) -> Result<(), Error> {
    log::info!("Starting shell ...");
    _ = shell(State::new(opts));
    Ok(())
}

fn shell(mut state: State) -> Result<State, String> {
    let prompt = state.prompt.clone();
    let mut history_file = dirs::data_dir().unwrap();
    history_file.push(Path::new("mdsplode"));
    match fs::create_dir_all(history_file.as_path()) {
        Ok(_) => (),
        Err(e) => log::error!("Couldn't create history file parent dirs: {:?}", e),
    }
    history_file.push(Path::new("cmd.history"));
    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history(&history_file).is_err() {
        log::debug!("No previous history.");
    }
    loop {
        log::debug!("Got state: {:?}", state);
        if state.show_banner {
            let mut colours = Colours::new();
            if !state.without_colour {
                // TODO: now that we're not formatting constants, let's use the owo-color library
                colours = Colours {
                    red: "\x1B[1;31m",
                    yellow_bold: "\x1B[1;33m",
                    yellow: "\x1B[33m",
                    green: "\x1B[32m",
                    blue: "\x1B[34m",
                    white: "\x1B[1;37m",
                    end: "\x1B[0m",
                };
            };
            state.show_banner = false;
            writer::msg(state.clone(), banner(colours).as_str())?;
            continue;
        }
        writer::flush()?;
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                match rl.add_history_entry(line.as_str()) {
                    Ok(_) => (),
                    Err(e) => log::error!("Could not update history file: {:?}", e),
                }
                state.history.push_front(line.clone());
                state.history.truncate(state.history_size);
                match cmd::dispatch(state.clone(), line.as_str()) {
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
            Err(ReadlineError::Interrupted) => {
                writer::msg(state.clone(), "Got ^c ...")?;
                state.quit = true;
                break;
            }
            Err(ReadlineError::Eof) => {
                log::info!("Got ^d ...");
                break;
            }
            Err(err) => {
                log::error!("Error: {:?}", err);
                break;
            }
        }
    }
    match rl.save_history(&history_file) {
        Ok(_) => (),
        Err(e) => log::warn!("Could not save history file: {:?}", e),
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
