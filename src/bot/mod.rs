mod calender_listing;
mod calender_available;
mod calender_unavailable;
mod booking_possible;

pub fn run() {
    let mut state = State::CalenderListing;
    loop { 
        state = run_state(state); 
    }
}

#[derive(Debug)]
pub enum State {
    CalenderListing,
    CalenderAvailable(u64),
    CalenderUnavailable,
    BookingPossible(u64)
}

fn run_state(state: State) -> State {
    log::info!("run state '{:?}'", state);
    match state {
        State::CalenderListing => calender_listing::run(),
        State::CalenderAvailable(id) => calender_available::run(id),
        State::CalenderUnavailable => calender_unavailable::run(),
        State::BookingPossible(id) => booking_possible::run(id)
    }
}



