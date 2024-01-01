use crate::cli::opts::Opts;

#[derive(Clone, Debug, Default)]
pub struct State {
    pub history: Vec<String>,
    pub prompt: String,
    pub quit: bool,
    pub result: String,
    pub show_banner: bool,
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
