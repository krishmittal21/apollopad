use crate::Terminal;
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to init terminal"),
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        //print!("\x1b[2j"); escape sequence -> \x1b is the escape character or 27 in decimal. j
        // -> to clear the screen 2-> argument that says clear the entire screen.
        // https://vt100.net/docs/vt100-ug/chapter3.html
        // termion allows to not write the escape sequence

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // goto using escape
                                                                          // sequence H command -> (row no, col no) at which to position the cursor, 1-based
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        io::stdout().flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?; // ? says if there is an error return it if not then
                                       // continue
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(()) // indicates that everything is okay, nothing has been returned
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
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
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
