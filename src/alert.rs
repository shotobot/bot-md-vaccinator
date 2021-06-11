use crate::shotobot::*;

#[derive(Debug)]
pub enum Alert {
    CalenderListIssue,
    CalenderStateIssue
}

pub fn alert(reason: Alert, msg: String) {
    match reason {
        Alert::CalenderListIssue => call_alert(&AlertApi::CalenderListIssue(msg)),
        Alert::CalenderStateIssue => call_alert(&AlertApi::CalenderStateIssue(msg))
    };
}