use std::sync::{Arc, mpsc, Mutex};
use std::thread;

mod snake;
use crate::snake::{screen, input};

use crossterm::terminal::{disable_raw_mode};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let lock_input_loop = Arc::clone(&counter);
    let lock_main_loop = Arc::clone(&counter);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        input::input_loop(tx, lock_input_loop);
    });

    screen::main_loop(60, 10, rx, lock_main_loop);
    disable_raw_mode();
}
