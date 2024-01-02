use std::collections::VecDeque;

use crate::cli::opts::Opts;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub history: VecDeque<String>,
    pub history_size: usize,
    pub in_file: String,
    pub out_file: String,
    pub parsed: String,
    pub prompt: String,
    pub quit: bool,
    pub result: String,
    pub show_banner: bool,
    pub source: String,
}

impl State {
    pub fn new(opts: Opts) -> State {
        State {
            history: VecDeque::new(),
            history_size: opts.history_size,
            prompt: opts.prompt,
            show_banner: true,
            ..State::default()
        }
    }
}
