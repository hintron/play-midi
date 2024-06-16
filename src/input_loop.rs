// Standard library imports
use std::thread::sleep;
use std::time::Duration;

// External imports

// Internal imports

pub fn input_sleep_loop(us: u64) {
    // Multiply everything by 10, so we can do fractional integer division
    // So a speedup of 11 is really 1.1x.
    let speedup = 11;
    let new_us = (us * 10) / speedup;
    sleep(Duration::from_micros(new_us));
}
