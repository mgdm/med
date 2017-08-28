use std::io;
use std::io::prelude::*;
use std::io::Stdout;
use libc::ioctl;
use libc::c_ushort;
use libc::TIOCGWINSZ;
use libc::STDOUT_FILENO;

pub struct Screen {
    stdout: Stdout,
    width: isize,
    height: isize
}

impl Screen {
    pub fn new(stdout: Stdout) -> Screen {
        let (width, height) = match Screen::get_terminal_size() {
            Ok(size) => size,
            _ => (0, 0)
        };


        Screen { stdout: stdout, width: width, height: height }
    }

    fn get_terminal_size() -> io::Result<(isize, isize)> {
        let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

        match r {
            0 => Ok((w.ws_col as isize, w.ws_row as isize)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Oh no"))
        }
    }

    pub fn clear(&self) {
        self.stdout.lock().write("\x1b[2J\x1b[H".as_bytes()).unwrap();
    }

    pub fn draw_rows(&self) {
        for _ in 1..self.height {
            self.stdout.lock().write("~\r\n".as_bytes()).unwrap();
        }
    }
}

#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}
