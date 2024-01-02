use crate::cli::opts::Opts;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub history: Vec<String>,
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
            prompt: opts.prompt.unwrap(),
            show_banner: true,
            ..State::default()
        }
    }
}
