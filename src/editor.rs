use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    exit: bool,
    terminal: Terminal,
    cursor_pos: Position,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                panic!("{}", err);
            }
            if self.exit {
                break;
            }
            if let Err(err) = self.keypress() {
                panic!("{}", err);
            }
        }
    }

    fn keypress(&mut self) -> Result<(), std::io::Error> {
        // read a key and process it
        let k = Terminal::read_key()?;
        match k {
            Key::Ctrl('q') => self.exit = true,
            Key::Up 
                | Key::Down 
                | Key::Left 
                | Key::Right 
                | Key::PageUp 
                | Key::PageDown
                | Key::End => self.move_cursor(k),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let mut x = self.cursor_pos.x;
        let mut y = self.cursor_pos.y;
        let size = self.terminal.size(); 
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left => {
                if x == 0 {
                    if y > 0 {
                        y = y.saturating_sub(1);
                        x = width - 1;
                    }
                }
                x = x.saturating_sub(1);
            },
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                } else {
                    if y < height {
                        y = y.saturating_add(1);
                        x = 0;
                    }
                }
            },
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_pos = Position { x, y }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::move_cursor(&Position { x: 0, y: 0 });
        if self.exit {
            Terminal::clear_screen();
            println!("exit\r");
        } else {
            self.draw_rows();
            Terminal::move_cursor(&self.cursor_pos);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Text Editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height - 1;
        for row in 0..height {
            Terminal::clear_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    pub fn default() -> Self {
        Self{
            exit: false,
            terminal: Terminal::default().expect("Couldn't initialize terminal!"),
            cursor_pos: Position { x: 0, y: 0 },
        }
    }
}
