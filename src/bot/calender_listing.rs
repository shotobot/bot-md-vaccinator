use crate::endpoint::{get, Endpoint};
use crate::pause::{pause, Pause};
use crate::bot::State;
use crate::alert::{alert, Alert};

use serde_json::Value;

pub fn run() -> State {
    match get(Endpoint::CalenderList) {
        Ok(json) => {
            match &json["Data"] {
                Value::Null => {
                    log::info!("*** no calendar data available ***");
                    pause(Pause::Short);
                    State::CalenderListing
                },
                Value::Array(vec) => {
                    if vec.is_empty() {
                        log::info!("*** no calendar available ***");
                        pause(Pause::Short);
                        State::CalenderListing
                    } else {
                        log::info!("*** calendar found ***");
                        let id = &vec[0]["Id"];
                        if id.is_number() {
                            State::CalenderAvailable(id.as_u64().unwrap())    
                        } else {
                            alert(Alert::CalenderListIssue, format!("json data: {}", json));
                            State::CalenderListing
                        }                        
                    }  
                },
                _ => {
                    alert(Alert::CalenderListIssue, format!("json data: '{}'", json));          
                    pause(Pause::Medium);
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