#![warn(clippy::all)]

use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn main() {
    println!("Hello, world!");
    // set terminal to raw mode, keep reference to stdout around
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        if c.is_control() {
            // ASCI 0-31, 127
            println!("{:?} \r", b)
        } else {
            // ASCI 32-126
            println!("{:?} ({})\r", b, c)
        }

        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
