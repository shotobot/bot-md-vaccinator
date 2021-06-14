use rand::Rng;
use std::{thread, time};

pub enum Pause {
    Short,
    Medium,
    Long
}

pub fn pause(amount: Pause) {
    let (lower, upper) = match amount {
        Pause::Short => (10, 20),  
        Pause::Medium => (30, 90),
        Pause::Long => (150, 450)
    };

    let sleep_sec = rand::thread_rng().gen_range(lower..upper);
    log::info!("suspend operations for {:.2} min", sleep_sec as f32 / 60.0);
    let duration = time::Duration::from_secs(sleep_sec);
    thread::sleep(duration);  
}