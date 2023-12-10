use color_eyre::Result;
use crossterm::event::KeyCode;
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

        match tui.events.next().unwrap() { 
            Event::Tick => { }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => {}
                }
            }
            _ => {}
        }

       
    }

    Ok(())
}
