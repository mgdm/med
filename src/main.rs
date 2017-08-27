extern crate libc;
extern crate termios;

use std::io;
use std::io::prelude::*;
use std::os::unix::io::RawFd;
use libc::iscntrl;

use termios::*;

fn enable_raw(fd: RawFd) -> Termios {
    let mut termios = Termios::from_fd(fd).unwrap();
    let original_termios = termios;

    termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    termios.c_oflag &= !OPOST;
    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);

    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;

    tcsetattr(fd, TCSAFLUSH, &termios).expect("Failed to set raw mode");
    tcflush(fd, TCIOFLUSH).expect("Failed to set raw mode");

    return original_termios;
}

fn disable_raw(fd: RawFd, termios: Termios) {
    tcsetattr(fd, TCSAFLUSH, &termios).expect("Failed to restore terminal state");
}

fn ctrl_key(c: u8) -> u8 {
    (c as u8 & 0x1f)
}

fn handle_char(c: u8) -> bool {
    let quit = ctrl_key(b'q');

    if c == quit {
        print!("Quitting");
        return true
    }

    unsafe {
        if iscntrl(c as i32) > 0 {
            print!("{}\r\n", c);
        } else {
            print!("{} ({})\r\n", c, c as char);
        }
    }

    false
}

fn main() {
    let stdin = io::stdin();
    let original_termios = enable_raw(0);

    'outer: loop {
        for char in stdin.lock().bytes() {
            let stop = match char {
                Ok(c) => handle_char(c),
                Err(e) => {
                    print!("{:?}\r\n", e);
                    true
                }
            };

            if stop {
                break 'outer;
            }
        }
    }

    disable_raw(0, original_termios);
}
