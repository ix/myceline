#![allow(dead_code)]
extern crate termios;

use std::io::prelude::*;
use std::io::{Result, stdin};

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
        cfmakeraw(&mut termios);
        termios.c_lflag |= ECHO;
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
    
    pub fn readline(&self, prompt: &str) -> Result<String> {
        let mut buf = String::new();

        loop {
            print!("{:?}", self.read_byte());
        }
        
        Ok(buf)
    }
}
