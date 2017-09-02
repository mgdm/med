use std::io;
use std::io::prelude::*;
use std::io::{Stdin, Stdout};
use libc::ioctl;
use libc::c_ushort;
use libc::TIOCGWINSZ;
use libc::STDOUT_FILENO;

use stdio::Stdio;

static CLEAR_SCREEN: &'static str = "\x1b[2J\x1b[H";
static CLEAR_LINE: &'static str = "\x1b[K";

static ENTER_ALTERNATE_BUFFER: &'static str = "\x1b[?1049h\x1b[H";
static EXIT_ALTERNATE_BUFFER: &'static str = "\x1b[?1049l";

static WELCOME: &'static str = "Welcome to med, version ";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Screen<'a> {
    stdio: &'a Stdio,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize
}

impl<'a> Screen<'a> {
    pub fn new(stdio: &Stdio) -> Screen {
        let (width, height) = match Screen::get_terminal_size() {
            Ok(size) => size,
            _ => (0, 0),
        };

        Screen {
            stdio: stdio,
            width: width,
            height: height,
            cursor_x: 10,
            cursor_y: 10
        }
    }

    fn get_terminal_size() -> io::Result<(usize, usize)> {
        let w = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

        match r {
            0 => Ok((w.ws_col as usize, w.ws_row as usize)),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get terminal size",
            )),
        }
    }

    pub fn enter_alternate_buffer(&self) {
        self.write(ENTER_ALTERNATE_BUFFER.as_bytes());
    }

    pub fn exit_alternate_buffer(&self) {
        self.write(EXIT_ALTERNATE_BUFFER.as_bytes());
    }

    pub fn clear(&self) {
        self.write(CLEAR_SCREEN.as_bytes())
    }

    pub fn get_rows(&self) -> String {
        if self.width < WELCOME.len() {
            let rows = (1..self.height)
                .map({ |_| "~\r\n" })
                .collect::<String>();

            return String::from(rows.trim_right());
        }

        let welcome_pos = self.height / 3;
        let remainder = self.height - welcome_pos - 1;
        let msg_start = (self.width - WELCOME.len()) / 2;

        let first = (1..welcome_pos)
            .map({ |_| "~\r\n" })
            .collect::<String>();

        let padding = (1..msg_start)
            .map({ |_| " " })
            .collect::<String>();

        let last = (welcome_pos..self.height)
            .map({ |_| "~\r\n" })
            .collect::<String>();

        format!("{}{}{}{}{}{}", first.trim_right(), padding, WELCOME, VERSION, "\r\n", last)
    }

    pub fn refresh(&self) {
        let str = format!("{}{}", CLEAR_SCREEN, self.get_rows());
        self.write(str.as_bytes());
    }

    fn write(&self, bytes: &[u8]) {
        self.stdio.stdout_lock().write(bytes).unwrap();
    }
}

#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}
