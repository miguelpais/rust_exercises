use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use ascii_canvas::{{AsciiCanvas, AsciiView}};

use super::direction::Direction;
use super::command::Command;

use ascii_canvas::style::DEFAULT;

use std::thread::sleep;
use std::time::Duration;

struct Screen {
    canvas: AsciiCanvas,
    cursorRow: usize,
    cursorColumn: usize,
    direction: Direction,
}

const ROWS: usize = 40;
const COLUMNS: usize = 80;

impl Screen {
    pub fn new() -> Screen {
        let mut canvas = AsciiCanvas::new(ROWS, COLUMNS);
        {
            let view: &mut dyn AsciiView = &mut canvas;
            view.draw_vertical_line(0..ROWS, 0);
            view.draw_vertical_line(0..ROWS, COLUMNS - 1);
            view.draw_horizontal_line(0, 0..COLUMNS);
            view.draw_horizontal_line(ROWS - 1, 0..COLUMNS);
        }
        Screen {
            canvas,
            cursorRow: ROWS / 2,
            cursorColumn: COLUMNS / 2,
            direction: Direction::LEFT,
        }
    }
}

pub fn main_loop(frames_per_second: u64, update_every_n_frames: u8, rx: Receiver<Command>, lock: Arc<Mutex<i32>>) {
    let mut screen = Screen::new();
    let frame_ttl_ms = 1000 / frames_per_second;
    let mut frame_counter = 0;

    loop {
        let new_command = rx.try_recv();
        match new_command {
            Ok(command) => {
                match command {
                    Command::UP => screen.direction = Direction::UP,
                    Command::DOWN => screen.direction = Direction::DOWN,
                    Command::LEFT => screen.direction = Direction::LEFT,
                    Command::RIGHT => screen.direction = Direction::RIGHT,
                    Command::NONE => (),
                    Command::EXIT => break
                }
            },
            _ => ()
        }
        if frame_counter > update_every_n_frames {
            frame_counter = 0;
            screen.canvas.write_char(screen.cursorRow, screen.cursorColumn, ' ', DEFAULT);
            match screen.direction {
                Direction::UP => screen.cursorRow = screen.cursorRow - 1,
                Direction::DOWN => screen.cursorRow = screen.cursorRow + 1,
                Direction::LEFT => screen.cursorColumn = screen.cursorColumn - 1,
                Direction::RIGHT => screen.cursorColumn = screen.cursorColumn + 1
            }
            if screen.cursorRow == (ROWS - 1) {
                screen.cursorRow = 1;
            } else if screen.cursorRow == 0 {
                screen.cursorRow = ROWS - 2;
            }
            if screen.cursorColumn == COLUMNS - 1 {
                screen.cursorColumn = 1
            } else if screen.cursorColumn == 0 {
                screen.cursorColumn = COLUMNS - 2
            }

            screen.canvas.write_char(screen.cursorRow, screen.cursorColumn, screen.direction.to_string(), DEFAULT);
        }
        frame_counter += 1;
        draw_screen(&screen, frame_ttl_ms, &lock);
    }
}

fn draw_screen(screen: &Screen, frame_ttl_ms: u64, lock: &Arc<Mutex<i32>>) {
    sleep(Duration::from_millis(frame_ttl_ms));
    let mut num = lock.lock().unwrap();
    print!("{}[2J", 27 as char);
    for row in screen.canvas.to_strings() {
        println!("{}", row);
    }
    *num = 1;
}
