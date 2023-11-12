use std::{f32, time::Instant};

pub struct Timer {
    frequency: u32,
    delta_step: u128,
    last_time: Instant,
}
impl Timer {
    pub fn new() -> Timer {
        let frequency = 4194304u32;
        // let frequency = 60u32;
        Timer {
            frequency,
            delta_step: Timer::step_length_nanosec(frequency),
            last_time: Instant::now(),
        }
    }
    pub fn start() {}
    pub fn stop() {}
    pub fn check_next_tick(&self) -> bool {
        self.last_time.elapsed().as_nanos() >= self.delta_step
    }
    pub fn next_tick(&mut self) {
        self.last_time = Instant::now();
    }
    fn step_length_nanosec(frequency: u32) -> u128 {
        ((1f32 / (frequency as f32)) * 1_000_000_000f32) as u128
    }
}
#[cfg(test)]
mod tests {
    use crate::timer::Timer;

    #[test]
    fn delta_step_test() {
        let timer = Timer::new();
        println!("tMIEME: {:?}", timer.delta_step);
        assert_eq!(238, timer.delta_step);
    }
}
