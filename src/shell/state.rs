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
    pub with_ext_line_ending: bool,
    pub without_colour: bool,
}

impl State {
    pub fn new(opts: Opts) -> State {
        State {
            history: VecDeque::new(),
            history_size: opts.history_size,
            prompt: opts.prompt,
            show_banner: !opts.no_banner,
            with_ext_line_ending: opts.etx_line_ending,
            without_colour: opts.no_colour,
            ..State::default()
        }
    }
}
