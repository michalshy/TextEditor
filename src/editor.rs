#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::execute;
use std::io::stdout;
use terminal::Terminal;
mod terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self{ should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode();
        Self::clear_screen()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
    fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}
