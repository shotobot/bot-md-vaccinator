use crate::bot::State;
use crate::pause::{pause, Pause};
use crate::endpoint::{get, Endpoint};
use crate::alert::{alert, Alert};
use crate::notify::{notify, Notify};

use serde_json::Value;

pub fn run(id: u64) -> State {
    match get(Endpoint::CalerderDetails(id)) {
        Ok(json) => {
            match json["Data"] {
                Value::Null => {
                    log::info!("*** no slots available ***");
                    pause(Pause::Short);
                    State::CalenderListing
                },
                Value::String(_) => {
                    log::info!("*** slots available ***");
                    notify(Notify::BookingActivated);
                    State::BookingPossible(id)
                },
                _ => {
                    log::info!("unexpected value found");
                    alert(Alert::CalenderStateIssue, format!("json data: '{}'", &json)); 
                    pause(Pause::Long);
                    State::CalenderListing
                }
            }
        },
        Err(err) => {
            alert(Alert::CalenderListIssue, format!("endpont call failed: '{}'", err)); 
            State::CalenderListing
        }
    }
}