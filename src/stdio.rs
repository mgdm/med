use std::io::{ Stdin, StdinLock, Stdout, StdoutLock };

pub struct Stdio {
    stdin: Stdin,
    stdout: Stdout
}

impl Stdio {
    pub fn new(stdin: Stdin, stdout: Stdout) -> Stdio {
        Stdio { stdin: stdin, stdout: stdout }
    }

    pub fn stdin_lock(&self) -> StdinLock {
        self.stdin.lock()
    }

    pub fn stdout_lock(&self) -> StdoutLock {
        self.stdout.lock()
    }
}
