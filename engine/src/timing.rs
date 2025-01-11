use std::time::Instant;

#[derive(Debug)]
pub struct Timer {
    prev: Instant,
    current: Instant,
    cumulative: f32,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            prev: Instant::now(),
            current: Instant::now(),
            cumulative: 0.0
        }
    }

    pub fn update(&mut self, steps_per_sec: u32) -> UpdateTimes {
        self.prev = self.current;
        self.current = Instant::now();
        let delta = self.current.duration_since(self.prev).as_secs_f32();
        self.cumulative += delta;

        let fixed_delta = 1.0 / (steps_per_sec as f32);
        let fixed_steps = (self.cumulative / fixed_delta).floor() as u64;
        self.cumulative %= fixed_delta;

        UpdateTimes {
            delta,
            fixed_delta,
            fixed_steps
        }
    }

    pub fn reset(&mut self) {
        self.prev = Instant::now();
        self.current = Instant::now();
        self.cumulative = 0.0;
    }
}

#[derive(Debug)]
pub struct UpdateTimes {
    pub delta: f32,
    pub fixed_delta: f32,
    pub fixed_steps: u64,
}