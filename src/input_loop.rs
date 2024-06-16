use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub fn input_sleep_loop(us: u64, is_running: Arc<AtomicBool>) {
    if !is_running.load(Ordering::SeqCst) {
        println!();
        send_all_notes_off();
        println!("Exiting playback early");
        std::process::exit(0);
    }

    // Multiply everything by 10, so we can do fractional integer division
    // So a speedup of 11 is really 1.1x.
    let speedup = 11;
    let new_us = (us * 10) / speedup;
    sleep(Duration::from_micros(new_us));
}

fn send_all_notes_off() {
    println!("ALL NOTES OFF");
}
