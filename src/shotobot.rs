use crate::conf::*;
use urlencoding::encode;

#[derive(Debug)]
pub enum AlertApi {
    CalenderListIssue(String),
    CalenderStateIssue(String)
}

#[derive(Debug)]
pub enum NotifyApi {
    BookingActivated,
    BookingDeactivated
}

pub fn call_alert(api: &AlertApi) {
    let conf = Conf::get();
    let (url, details) = match api {
        AlertApi::CalenderListIssue(details) => ((&conf.alert_api.calender_list_issue).replace("$(details)", &encode(&details)), details),
        AlertApi::CalenderStateIssue(details) => ((&conf.alert_api.calender_state_issue).replace("$(details)", &encode(&details)), details)
    };

    log::warn!("raise alert '{:?}': '{}' ({})", api, details, url);
    call("notify api", url.to_string());
}

pub fn call_notify(api: NotifyApi) {
    let conf = Conf::get();
    let url = match api {
        NotifyApi::BookingActivated => &conf.notify_api.booking_activated,
        NotifyApi::BookingDeactivated => &conf.notify_api.booking_deactivated
    };
    log::info!("raise notification '{:?}' ({})", api, url);
    call("notify api", url.to_string());
}

fn call(name: &str, url: String) {
    match reqwest::blocking::get(&url) {
        Ok(_) => log::info!("{}: succesfully called ({})", name, url),
        Err(err) => log::error!("{} call failed: '{}' ({})", name, err, url)
    }
}