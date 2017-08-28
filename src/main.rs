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
    unsafe {
        iscntrl(c as i32) > 0
    }
}


fn handle_char(c: u8) -> bool {
    let quit = ctrl_key(b'q');

    if c == quit {
        print!("Quitting");
        return true
    }

    if is_ctrl(c) {
        print!("{}\r\n", c);
    } else {
        print!("{} ({})\r\n", c, c as char);
    }

    false
}

fn main() {
    let mut input = Input::new(0, io::stdin());
    let screen = Screen::new(io::stdout());
    let mut stop = false;

    input.enable_raw();

    while !stop {
        screen.clear();
        screen.draw_rows();
        let char = input.read_key();
        stop = handle_char(char);
    }

    input.disable_raw();
}
