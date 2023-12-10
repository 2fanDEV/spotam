use std::str::pattern::StrSearcher;

use color_eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

mod application;
pub mod event;
pub mod tui;
mod ui;

use application::Application;
use event::{Event, EventHandler};
use tui::TUI;

fn main() -> Result<()> {
    let services = ["Spotify", "Apple Music"];
    let mut app = Application::new(services.to_vec());

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = TUI::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        let services_not_logged_in = app
            .get_streaming_services()
            .iter()
            .filter(|service| service.is_user_logged_in()).collect::<Vec<_>>();
        tui.draw(&mut app)?;
    }

    Ok(())
}
