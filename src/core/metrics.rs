use std::time::Instant;

pub struct ExecutionMetrics {
    start_time: Instant,
    pub script_length: usize,
}

impl ExecutionMetrics {
    pub fn new(script_length: usize) -> Self {
        Self {
            start_time: Instant::now(),
            script_length,
        }
    }

    pub fn execution_time_ms(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
}