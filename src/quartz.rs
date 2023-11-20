use std::{
    f32, thread,
    time::{Duration, Instant},
};

pub struct Quartz {
    frequency: u32,
    delta_step: u128,
    last_time: Instant,
}
impl Quartz {
    pub fn new() -> Quartz {
        // True value
        let frequency = 4194304u32;
        // Slower value
        // let frequency = 60000u32;
        Quartz {
            frequency,
            delta_step: Quartz::step_length_nanosec(frequency),
            last_time: Instant::now(),
        }
    }
    pub fn start() {}
    pub fn stop() {}
    pub fn wait_till_next_tick(&self) {
        let elapsed_time = self.last_time.elapsed().as_nanos();
        if elapsed_time < self.delta_step {
            let sleep_time = self.delta_step - elapsed_time;
            thread::sleep(Duration::from_nanos(sleep_time as u64));
        }
    }
    // TODO: maybe, since every action takes a multiple of 4 ticks make
    // everything take 4 time less tick and make tick 4 time more spaced
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
    use crate::quartz::Quartz;


    #[test]
    fn delta_step_test() {
        let timer = Quartz::new();
        println!("tMIEME: {:?}", timer.delta_step);
        assert_eq!(238, timer.delta_step);
    }
}
