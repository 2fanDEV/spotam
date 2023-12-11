use std::panic::UnwindSafe;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{backend::CrosstermBackend, Terminal};

mod application;
pub mod event;
pub mod tui;
pub mod update;
mod ui;

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
        let mut services_not_logged_in = app.streaming_services
            .iter()
            .filter(|service| service.is_user_logged_in())
            .collect::<Vec<_>>();
        let x = 0; 
        let mut y = 0;
        // tui.draw(&mut app, 0, 0);
        services_not_logged_in.iter().for_each(|&service| {
            tui.draw2(service.get_name(), x, y);
        });

        update(&mut app, tui.events.next())?;
        
       
    }

    Ok(())
}
