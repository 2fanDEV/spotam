use std::panic::UnwindSafe;

use color_eyre::Result;
use crossterm::{event::{KeyCode, KeyEvent}, execute, cursor::{SavePosition, MoveTo, EnableBlinking, DisableBlinking, RestorePosition}};
use ratatui::{backend::CrosstermBackend, Terminal};

mod application;
pub mod event;
pub mod tui;
mod ui;
pub mod update;

use application::Application;
use event::{Event, EventHandler};
use tui::TUI;
use update::update;

fn main() -> Result<()> {
    let services = ["Spotify", "Apple Music"];
    let mut app = Application::new(services.to_vec());

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = TUI::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        let x = 0;
        let mut y = 0;
        tui.draw(&mut app);
        update(&mut app, tui.events.next())?;
    }

    tui.exit()?;


    Ok(())
}
