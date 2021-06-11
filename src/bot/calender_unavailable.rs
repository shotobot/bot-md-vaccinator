use crate::bot::State;

pub fn run() -> State {
    log::info!("calender unavailable");
    State::CalenderUnavailable
}