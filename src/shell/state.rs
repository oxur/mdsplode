use std::collections::VecDeque;

use crate::cli::{
    self,
    opts::{Format, Opts},
};

#[derive(Clone, Debug, Default)]
pub struct State {
    pub device: String,
    pub format: Format,
    pub history: VecDeque<String>,
    pub history_size: usize,
    pub in_file: String,
    pub out_file: String,
    pub parsed: String,
    pub prompt: String,
    pub quit: bool,
    pub result: String,
    pub source: String,
    pub show_banner: bool,
    pub without_colour: bool,
}

impl State {
    pub fn new(opts: Opts) -> State {
        State {
            device: opts.device.unwrap(),
            format: opts.format,
            history: VecDeque::new(),
            history_size: opts.history_size,
            prompt: opts.prompt,
            show_banner: !opts.no_banner,
            without_colour: opts.no_colour,
            ..State::default()
        }
    }

    pub fn is_stderr(&self) -> bool {
        self.device == cli::stderr().unwrap()
    }
}
