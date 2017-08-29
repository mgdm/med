use std::io;
use std::io::prelude::*;
use std::io::{Stdin, Stdout};
use libc::ioctl;
use libc::c_ushort;
use libc::TIOCGWINSZ;
use libc::STDOUT_FILENO;

static CLEAR_SCREEN: &'static str = "\x1b[2J\x1b[H";
static CLEAR_LINE: &'static str = "\x1b[K";

static POSITION_REPORT: &'static str = "\x1b[6n\r\n";

static SHOW_CURSOR: &'static str = "\x1b[?25h";
static HIDE_CURSOR: &'static str = "\x1b[?25l";

static WELCOME: &'static str = "Welcome to med, version ";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Screen {
    stdin: Stdin,
    stdout: Stdout,
    width: usize,
    height: usize,
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

    pub fn get_cursor_position(&self) -> io::Result<(usize, usize)> {
        let mut buf = String::new();

        self.write(POSITION_REPORT.as_bytes());

        let (w, h): (usize, usize);
        match self.stdin.lock().read_to_string(&mut buf) {
            Ok(s) => {
                scan!(buf.bytes() => "\x1b[{};{}R", w, h);
                Ok((w, h))
            },
            _ => Err(
                io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to get cursor position",
                    ))
        }
    }

    pub fn clear(&self) {
        self.write(CLEAR_SCREEN.as_bytes())
    }

    pub fn get_rows(&self) -> String {
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

    pub fn show_cursor(&self) {
        self.write(SHOW_CURSOR.as_bytes());
    }

    pub fn hide_cursor(&self) {
        self.write(HIDE_CURSOR.as_bytes());
    }

    pub fn refresh(&self) {
        let str = format!("{}{}{}{}", HIDE_CURSOR, CLEAR_SCREEN, self.get_rows(), SHOW_CURSOR);
        self.write(str.as_bytes());
    }

    fn write(&self, bytes: &[u8]) {
        self.stdout.lock().write(bytes).unwrap();
    }
}

#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,
}
