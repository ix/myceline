#![allow(dead_code)]
extern crate termios;

use std::io::prelude::*;
use std::io::{Result, stdin, stdout};

use termios::*;

pub struct Editor {
    initial: Termios
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            initial: Termios::from_fd(0).unwrap().clone()
        }
    }

    fn start_raw(&self) {
        let mut termios = self.initial.clone();
        termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_iflag &= !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
        termios.c_cflag &= !(CSIZE | PARENB);
        termios.c_cflag |= CS8;
        termios.c_oflag &= !(OPOST);
        termios.c_cc[VMIN] = 1;
        termios.c_cc[VTIME] = 0;
        tcsetattr(0, TCSANOW, &mut termios).unwrap();
    }

    fn end_raw(&self) {
        tcsetattr(0, TCSANOW, &self.initial);
    }

    fn read_byte(&self) -> Option<Result<u8>> {
        self.start_raw();
        
        let mut handle = stdin();
        let mut reader = handle.lock().bytes();

        let result = reader.next();
        
        self.end_raw();
        
        result
    }
    
    pub fn readline(&self, prompt: &str) -> Option<String> {
        let mut buf = String::new();
        print!("{} ", prompt);
        stdout().flush();
        
        loop {

            if let Some(byte) = self.read_byte() {
                match byte {
                    Ok(3)  => break,
                    Ok(13) => break,
                    Ok(b @ 32 ... 126)  => {
                        print!("{}", b as char);
                        buf.push(b as char);
                    },                    
                    _      => {}
                }
            }
            
            stdout().flush();
        }

        if buf != "" {
            Some(buf)
        }

        else {
            None
        }
    }
}
