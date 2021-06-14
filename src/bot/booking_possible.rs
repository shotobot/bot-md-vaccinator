use std::time::Instant;
use crate::bot::State;
use crate::alert::{alert, Alert};
use crate::notify::{notify, Notify};
use crate::endpoint::{get, Endpoint};
use crate::pause::{pause, Pause};

use serde_json::Value;

pub fn run(id: u64, open: Instant, notified: bool) -> State {
    match get(Endpoint::CalerderDetails(id)) {
        Ok(json) => {
            match json["Data"] {
                Value::Null => {
                    log::info!("*** booking no longer possible ***");
                    notify(Notify::BookingDeactivated);
                    State::CalenderListing
                },
                Value::String(_) => {
                    log::info!("*** booking still possible ***");
                    let shall_notifiy = Instant::now().duration_since(open).as_secs() > 120;
                    if shall_notifiy && !notified {
                        notify(Notify::BookingActivated);
                    }
                    pause(Pause::Short);
                    State::BookingPossible(id, open, shall_notifiy)
                },
                _ => {
                    log::info!("unexpected value found");
                    notify(Notify::BookingDeactivated);
                    alert(Alert::CalenderStateIssue, format!("json data: '{}'", &json));
                    pause(Pause::Long);
                    State::CalenderListing
                }
            }
        },
        Err(err) => {
            alert(Alert::CalenderStateIssue, format!("endpont call failed: '{}'", err)); 
            State::CalenderListing
        }
    }
}