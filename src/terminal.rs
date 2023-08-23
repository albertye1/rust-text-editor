use crate::Position;
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {size: Size { width: size.0, height: size.1 }, stdout: stdout().into_raw_mode()?}, )
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn move_cursor(pos: &Position) {
        let x = ((*pos).x.saturating_add(1)) as u16; // refuses to add more lines instead of overflowing
        let y = ((*pos).y.saturating_add(1)) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
