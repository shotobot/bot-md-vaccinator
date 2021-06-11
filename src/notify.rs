use crate::shotobot::*;

#[derive(Debug)]
pub enum Notify {
    BookingActivated,
    BookingDeactivated
}

pub fn notify(reason: Notify) {
    match reason {
        Notify::BookingActivated => call_notify(NotifyApi::BookingActivated),
        Notify::BookingDeactivated => call_notify(NotifyApi::BookingDeactivated)
    }
}
