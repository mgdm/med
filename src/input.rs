use std::io::prelude::*;
use std::io::Stdin;
use std::os::unix::io::RawFd;
use termios::*;

use stdio::Stdio;

pub struct Input<'a> {
    fd: RawFd,
    original_termios: Termios,
    stdio: &'a Stdio,
}

impl<'a> Input<'a> {
    pub fn new(fd: RawFd, stdio: &'a Stdio) -> Input {
        let termios = Termios::from_fd(fd).unwrap();
        Input {
            fd: fd,
            stdio: stdio,
            original_termios: termios,
        }
    }

    pub fn enable_raw(&mut self) {
        let mut termios = self.original_termios;

        termios.c_cflag |= CS8;
        termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_oflag &= !OPOST;
        termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);

        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 1;

        tcsetattr(self.fd, TCSAFLUSH, &termios).expect("Failed to set raw mode");
        tcflush(self.fd, TCIOFLUSH).expect("Failed to set raw mode");
    }

    pub fn disable_raw(&self) {
        tcsetattr(self.fd, TCSAFLUSH, &self.original_termios)
            .expect("Failed to restore terminal state");
    }

    pub fn read_key(&self) -> u8 {
        match self.stdio.stdin_lock().bytes().next() {
            Some(c) => c.unwrap(),
            None => 0,
        }
    }
}
