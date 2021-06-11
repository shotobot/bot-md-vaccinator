use std::env;

pub struct AlertApiUrls {
    pub calender_list_issue: String,
    pub calender_state_issue: String,
}

pub struct NotifyApiUrls {
    pub booking_activated: String,
    pub booking_deactivated: String
}

pub struct Conf {
    pub alert_api: AlertApiUrls,
    pub notify_api: NotifyApiUrls
}

impl Conf {
    pub fn get() -> Conf {
        let conf = Conf {
            alert_api: AlertApiUrls {
                calender_list_issue: envvar("CALENDER_LIST_ISSUE_URL"),
                calender_state_issue: envvar("CALENDER_STATE_ISSUE_URL")
            },
            notify_api: NotifyApiUrls {
                booking_activated: envvar("BOOKING_ACTIVATED_URL"),
                booking_deactivated: envvar("BOOKING_DEACTIVATED_URL")
            }
        };
        if [
            &conf.alert_api.calender_list_issue,
            &conf.alert_api.calender_state_issue,
            &conf.notify_api.booking_activated,
            &conf.notify_api.booking_deactivated,
           ].iter().any(|x| x.is_empty()) {
               panic!("shotobot API misconfigured: check environment variables")
           } else {
               conf
           }
    }
}

pub fn verify() {
    Conf::get();
}

fn envvar(name: &str) -> String {
    let empty = "".to_string();
    env::var(name).unwrap_or(empty)
}