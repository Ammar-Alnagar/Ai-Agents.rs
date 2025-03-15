// src/metrics.rs
use std::time::Instant;

pub struct Metrics {
    start_time: Instant,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics { start_time: Instant::now() }
    }

    pub fn log(&self, label: &str) {
        println!("[{}] Took {:?}", label, self.start_time.elapsed());
    }
}
