use std::io;
use std::io::prelude::*;
use std::io::{ Stdin, StdinLock, Stdout, StdoutLock };

use stdio::Stdio;

static POSITION_REPORT: &'static str = "\x1b[6n\r\n";
static SHOW_CURSOR: &'static str = "\x1b[?25h";
static HIDE_CURSOR: &'static str = "\x1b[?25l";

pub struct Cursor {
    x: usize,
    y: usize,
    x_limit: usize,
    y_limit: usize,
    stdio: Stdio
}

impl Cursor {
    pub fn new(x_limit: usize, y_limit: usize, stdio: Stdio) -> Cursor {
        Cursor { x: 1,
        y: 1,
        x_limit: x_limit, y_limit: y_limit,
        stdio: stdio
        }
    }

    pub fn get_position(&self) -> io::Result<(usize, usize)> {
        let mut buf = String::new();

        self.write(POSITION_REPORT.as_bytes());

        let (w, h): (usize, usize);
        match self.stdio.stdin_lock().read_to_string(&mut buf) {
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

    pub fn show(&self) {
        self.write(SHOW_CURSOR.as_bytes());
    }

    pub fn hide(&self) {
        self.write(HIDE_CURSOR.as_bytes());
    }

    fn move_to(&self, x: u8, y: u8) {
        self.write(format!("\x1b[{};{}H", y, x).as_bytes());
    }
    
    fn write(&self, bytes: &[u8]) {
        self.stdio.stdout_lock().write(bytes).unwrap();
    }
}
