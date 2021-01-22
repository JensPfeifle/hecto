use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        println!("^q to exit");
        // set terminal to raw mode, keep reference to stdout around
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(err) = self.process_keypress() {
                die(err);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char(c) => println!("{}\r", c),
            _ => (),
        }
        Ok(())
    }

    pub fn default() -> Self {
        Editor { should_quit: false }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!(e)
}
