use std::{panic, io};

use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{event::EventHandler, application::Application, ui};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct TUI {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}

impl TUI {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()>{
        Self::reset()?;
        self.terminal.show_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self,  app: &mut Application, x: u16, y: u16) { 
        self.terminal.draw(|frame| ui::render(app, frame, x, y));
    }

    pub fn draw2(&mut self,  name: &str, x: u16, y: u16) { 
        self.terminal.draw(|frame| ui::render2(name, frame, x, y));
    }

}


