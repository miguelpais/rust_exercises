use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use ascii_canvas::{{AsciiCanvas, AsciiView}};
extern crate crossterm;

use crossterm::event::{read, Event, KeyCode};
use crossterm::event::{poll};

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use ascii_canvas::style::DEFAULT;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const ROWS: usize = 40;
const COLUMNS: usize = 80;

#[derive(PartialEq, Clone, Copy)]
enum Command {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
    EXIT
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    fn to_string(&self) -> char {
        match self {
            Direction::UP => '‖',
            Direction::DOWN => '‖',
            Direction::LEFT => '═',
            Direction::RIGHT => '═',
        }
    }
}

fn capture_command() -> Option<Command> {
    let mut new_command = None;
    enable_raw_mode();
    let status = poll(Duration::from_millis(10));
    if status.is_ok() && status.unwrap() {
        let event = read();
        if event.is_ok() {
            let specific = event.unwrap();
            if specific == Event::Key(KeyCode::Right.into()) {
                new_command = Some(Command::RIGHT)
            }
            else if specific == Event::Key(KeyCode::Left.into()) {
                new_command = Some(Command::LEFT)
            }
            else if specific == Event::Key(KeyCode::Down.into()) {
                new_command = Some(Command::DOWN)
            }
            else if specific == Event::Key(KeyCode::Up.into()) {
                new_command = Some(Command::UP)
            }
            else if specific == Event::Key(KeyCode::Esc.into()) {
                new_command = Some(Command::EXIT)
            }
        }
    }
    disable_raw_mode();
    new_command
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut canvas = AsciiCanvas::new(ROWS, COLUMNS);
    {
        let view: &mut dyn AsciiView = &mut canvas;
        view.draw_vertical_line(0..ROWS, 0);
        view.draw_vertical_line(0..ROWS, (COLUMNS-1));
        view.draw_horizontal_line(0, 0..COLUMNS);
        view.draw_horizontal_line((ROWS-1), 0..COLUMNS);
    }
    let mut pos = (ROWS/2, COLUMNS/2);
    let mut direction = Direction::LEFT;
    let counter_tx1 = Arc::clone(&counter);
    let counter_tx2 = Arc::clone(&counter);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            {
                let mut num = counter_tx1.lock().unwrap();
                *num = 1;
                match capture_command() {
                    Some(new_command) => tx.send(new_command).unwrap(),
                    _ => ()
                }
            }
            sleep(Duration::from_millis(5));
        }
    });
    let mut frame_counter = 0;
    loop {
        sleep(Duration::from_millis(16));
        let new_command = rx.try_recv();
        match new_command{
            Ok(command) => {
                match command {
                    Command::UP => direction = Direction::UP,
                    Command::DOWN => direction = Direction::DOWN,
                    Command::LEFT => direction = Direction::LEFT,
                    Command::RIGHT => direction = Direction::RIGHT,
                    Command::NONE => (),
                    Command::EXIT => break
                }
            },
            _ => ()
        }
        if frame_counter > 10 {
            frame_counter = 0;
            canvas.write_char(pos.0, pos.1, ' ', DEFAULT);
            match direction {
                Direction::UP => pos = (pos.0 - 1, pos.1),
                Direction::DOWN => pos = (pos.0 + 1, pos.1),
                Direction::LEFT => pos = (pos.0, pos.1 - 1),
                Direction::RIGHT => pos = (pos.0, pos.1 + 1)
            }
            if pos.0 == (ROWS - 1) {
                pos.0 = 1;
            }
            else if pos.0 == 0 {
                pos.0 = ROWS - 2;
            }
            if pos.1 == COLUMNS - 1 {
                pos.1 = 1
            }
            else if pos.1 == 0 {
                pos.1 = COLUMNS - 2
            }

            canvas.write_char(pos.0, pos.1, direction.to_string(), DEFAULT);

        }
        frame_counter += 1;
        let mut num = counter_tx2.lock().unwrap();
        print!("{}[2J", 27 as char);
        for row in &canvas.to_strings() {
            println!("{}", row);
        }
        *num = 1;
    }
    disable_raw_mode();
}
