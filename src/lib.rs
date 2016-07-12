#![allow(dead_code)]
extern crate termios;

use std::io::prelude::*;
use std::io::{Result, stdin, stdout};
use std::cmp::{min, max};

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
    let mut handle = stdout();
    let mut buf = String::new();
    let offset = prompt.len();
    let mut index: usize = offset;

    loop {
      if let Some(byte) = self.read_byte() {
        match byte {
          // Ctrl-C
          Ok(3) => break,
          // Ctrl-A
          Ok(1) => index = offset,
          // Ctrl-E
          Ok(5) => index = (offset + buf.len()),
          // Ctrl-K
          Ok(11) => {
            buf.truncate(index - offset);
            print!("\u{001b}[0K");
          }
          // Return
          Ok(13) => break,
          // Handle cursor movement.
          // 27 is ESC
          Ok(27) => {
            // 91 is [
            if let Some(Ok(91)) = self.read_byte() {
              match self.read_byte() {
                // Left
                Some(Ok(68)) => {
                  if index > offset {
                    index = index - 1
                  }
                },
                // Right
                Some(Ok(67)) => {
                  if index < (offset + buf.len()) {
                    index = index + 1
                  }            
                },
                _ => {}
              }
            }
          },
          Ok(b @ 32 ... 126)  => {
            buf.push(b as char);
            index = index + 1
          },
          _ => {}
        }
      }

      print!("\u{001b}[1000D");
      print!("{}", prompt);
      print!("{}", buf);
      print!("\u{001b}[1000D");
    
      if index >= offset {
        print!("\u{001b}[{}C", index.to_string());
      }
      
      handle.flush();
    }
        
    if buf != "" {
      Some(buf)
    }

    else {
      None
    }
  }
}
