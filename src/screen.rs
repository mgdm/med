use std::io;
use std::io::prelude::*;
use std::io::{Stdin, Stdout};
use libc::ioctl;
use libc::c_ushort;
use libc::TIOCGWINSZ;
use libc::STDOUT_FILENO;

use std::str;


pub struct Screen {
    stdin: Stdin,
    stdout: Stdout,
    width: isize,
    height: isize,
}

impl Screen {
    pub fn new(stdin: Stdin, stdout: Stdout) -> Screen {
        let (width, height) = match Screen::get_terminal_size() {
            Ok(size) => size,
            _ => (0, 0),
        };

        Screen {
            stdin: stdin,
            stdout: stdout,
            width: width,
            height: height,
        }
    }

    fn get_terminal_size() -> io::Result<(isize, isize)> {
        let w = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

        match r {
            0 => Ok((w.ws_col as isize, w.ws_row as isize)),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get terminal size",
            )),
        }
    }

    pub fn get_cursor_position(&self) -> io::Result<(isize, isize)> {
        let mut buf = String::new();

        self.stdout.lock().write("\x1b[6n\r\n".as_bytes()).unwrap();

        let (w, h): (isize, isize);
        match self.stdin.lock().read_to_string(&mut buf) {
            Ok(s) => {
                scan!(buf.bytes() => "\x1b[{};{}R", w, h);
                Ok((w, h))
            },
            _ => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to get cursor position",
                    )),
        }
    }

    pub fn clear(&self) {
        self.stdout
            .lock()
            .write("\x1b[2J\x1b[H".as_bytes())
            .unwrap();
    }

    pub fn draw_rows(&self) {
        let rows = (1..self.height - 5)
            .map({ |_| "~\r\n" })
            .collect::<String>();
        self.stdout.lock().write(rows.as_bytes()).unwrap();
    }
}

#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}
