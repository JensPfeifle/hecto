#![warn(clippy::all)]

use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!(e)
}

fn main() {
    println!("Hello, world!");
    // set terminal to raw mode, keep reference to stdout around
    let _stdout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        // ASCI 0-31, 127
                        println!("{:?} \r", c as u8)
                    } else {
                        // ASCI 32-126
                        println!("{:?} ({})\r", c as u8, c)
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => die(err),
        }
    }
}
