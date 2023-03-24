use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use ascii_canvas::{{AsciiCanvas, AsciiView}};

use super::direction::Direction;
use super::command::Command;

use ascii_canvas::style::DEFAULT;

use std::thread::sleep;
use std::time::Duration;

pub struct Screen {
    canvas: AsciiCanvas,
    cursor_row: usize,
    cursor_column: usize,
    width: usize,
    height: usize,
    direction: Direction,
}


impl Screen {
    pub fn new(height: usize, width: usize) -> Screen {
        let mut canvas = AsciiCanvas::new(height, width);
        {
            let view: &mut dyn AsciiView = &mut canvas;
            view.draw_vertical_line(0..height, 0);
            view.draw_vertical_line(0..height, width - 1);
            view.draw_horizontal_line(0, 0..width);
            view.draw_horizontal_line(height - 1, 0..width);
        }
        Screen {
            canvas,
            width,
            height,
            cursor_row: height / 2,
            cursor_column: width / 2,
            direction: Direction::LEFT,
        }
    }

    pub fn main_loop(&mut self, frames_per_second: u64, update_every_n_frames: u8, rx: Receiver<Command>, lock: Arc<Mutex<i32>>) {
        let frame_ttl_ms = 1000 / frames_per_second;
        let mut frame_counter = 0;

        loop {
            let new_command = rx.try_recv();
            match new_command {
                Ok(command) => {
                    match command {
                        Command::UP => self.direction = Direction::UP,
                        Command::DOWN => self.direction = Direction::DOWN,
                        Command::LEFT => self.direction = Direction::LEFT,
                        Command::RIGHT => self.direction = Direction::RIGHT,
                        Command::NONE => (),
                        Command::EXIT => break
                    }
                },
                _ => ()
            }
            if frame_counter > update_every_n_frames {
                frame_counter = 0;
                self.update_position();

            }
            frame_counter += 1;
            self.draw_screen(frame_ttl_ms, &lock);
        }
    }

    fn update_position(&mut self) {
        self.canvas.write_char(self.cursor_row, self.cursor_column, ' ', DEFAULT);
        match self.direction {
            Direction::UP => self.cursor_row = self.cursor_row - 1,
            Direction::DOWN => self.cursor_row = self.cursor_row + 1,
            Direction::LEFT => self.cursor_column = self.cursor_column - 2,
            Direction::RIGHT => self.cursor_column = self.cursor_column + 2
        }
        if self.cursor_row == (self.height - 1) {
            self.cursor_row = 1;
        } else if self.cursor_row == 0 {
            self.cursor_row = self.height - 2;
        }
        if self.cursor_column >= self.width - 1 {
            self.cursor_column = 1
        } else if self.cursor_column <= 1 {
            self.cursor_column = self.width - 2
        }

        self.canvas.write_char(self.cursor_row, self.cursor_column, self.direction.to_string(), DEFAULT);
    }

    fn draw_screen(&self, frame_ttl_ms: u64, lock: &Arc<Mutex<i32>>) {
        sleep(Duration::from_millis(frame_ttl_ms));
        let mut num = lock.lock().unwrap();
        print!("{}[2J", 27 as char);
        for row in self.canvas.to_strings() {
            println!("{}", row);
        }
        *num = 1;
    }
}
