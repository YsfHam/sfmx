use std::time::{Instant, Duration};

pub struct Timer {
    start: Instant
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now()
        }
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.start
    }

    pub fn restart(&mut self) -> Duration {
        let elapsed = self.elapsed();
        *self = Timer::new();

        elapsed
    }
}