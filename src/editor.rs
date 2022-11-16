extern crate termion;
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("command:{:?}\r", c);
                        } else {
                            println!("c as u8:{:?}, c: {}\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => panic!("{}", err),
            }
        }
    }
}
