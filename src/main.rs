#[macro_use]
extern crate text_io;


extern crate libc;
extern crate termios;

mod input;
mod screen;

use std::io;
use libc::iscntrl;

use input::Input;
use screen::Screen;

fn ctrl_key(c: u8) -> u8 {
    (c as u8 & 0x1f)
}

fn is_ctrl(c: u8) -> bool {
    unsafe { iscntrl(c as i32) > 0 }
}

fn handle_char(c: u8) -> bool {
    let quit = ctrl_key(b'q');

    if c == quit {
        print!("Quitting");
        return true;
    }

    false
}

fn main() {
    let mut input = Input::new(0, io::stdin());
    let screen = Screen::new(io::stdin(), io::stdout());
    let mut stop = false;

    input.enable_raw();

    let (x, y) = screen
        .get_cursor_position()
        .expect("Failed to get cursor position");

    print!("Position: ({}, {})\r\n", x, y);

    while !stop {
        screen.refresh();
        let char = input.read_key();
        stop = handle_char(char);
    }

    input.disable_raw();
}
