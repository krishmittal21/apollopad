use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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

        //print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1)); // goto using escape
        // sequence H command -> (row no, col no) at which to position the cursor, 1-based
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?; // ? says if there is an error return it if not then
                                                 // continue
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(()) // indicates that everything is okay, nothing has been returned
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            println!("~\r");
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
