#[derive(Clone, Debug, Default)]
pub struct State {
    pub quit: bool,
    pub result: String,
}

impl State {
    pub fn new() -> State {
        State { ..State::default() }
    }
}
