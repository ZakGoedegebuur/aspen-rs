use std::time::Instant;

pub struct Timer {
    prev: Instant,
    current: Instant,
    cumulative: f64,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            prev: Instant::now(),
            current: Instant::now(),
            cumulative: 0.0
        }
    }

    pub fn update(&mut self, steps_per_sec: u64) -> UpdateTimes {
        self.prev = self.current;
        self.current = Instant::now();
        let delta = self.current.duration_since(self.prev).as_secs_f64();
        self.cumulative += delta;

        let fixed_delta = 1.0 / (steps_per_sec as f64);
        let fixed_steps = (self.cumulative / fixed_delta).floor() as u64;
        self.cumulative %= fixed_delta;

        UpdateTimes {
            delta,
            fixed_delta,
            fixed_steps
        }
    }
}

#[derive(Debug)]
pub struct UpdateTimes {
    pub delta: f64,
    pub fixed_delta: f64,
    pub fixed_steps: u64,
}