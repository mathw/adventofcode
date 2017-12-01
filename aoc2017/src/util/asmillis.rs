use std::time::Duration;

pub trait AsMillis {
    fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
    fn as_millis(&self) -> u64 {
        let seconds = self.as_secs();
        let nanos = self.subsec_nanos();
        let millis = nanos / 1000000;
        seconds * 1000 + (millis as u64)
    }
}