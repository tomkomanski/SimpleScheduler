use std::time::Instant;

pub struct Trigger {
    pub interval: u64,
    pub last_shot: Option<Instant>,
    pub paused: bool,
}

impl Trigger {
    pub fn new(interval_sec: u16) -> Trigger {
        let trigger: Trigger = Trigger {
            interval: interval_sec as u64,
            paused: false,
            last_shot: None,
        };

        return trigger;
    }
}