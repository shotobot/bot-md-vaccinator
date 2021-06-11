use rand::Rng;
use std::{thread, time};
use chrono::{Timelike, Utc};

pub enum Pause {
    Short,
    Medium,
    Long
}

pub fn pause(amount: Pause) {
    let night_mode = is_night();
    if night_mode {
        log::info!("enter night mode");
    } else {
        log::info!("enter day mode");
    }
    let night_factor = if night_mode { 5 } else { 1 };

    let (lower, upper) = match amount {
        Pause::Short => (10, 20 * night_factor),  
        Pause::Medium => (30, 90 * night_factor),
        Pause::Long => (150, 450 * night_factor)
    };

    let sleep_sec = rand::thread_rng().gen_range(lower..upper);
    log::info!("suspend operations for {:.2} min", sleep_sec as f32 / 60.0);
    let duration = time::Duration::from_secs(sleep_sec);
    thread::sleep(duration);  
}

fn is_night() -> bool {
    let now = Utc::now();
    let hour = now.hour();
    hour >= 23 && hour <= 3
}