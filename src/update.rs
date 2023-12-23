use crate::{application::Application, event::Event};
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};

pub fn update(app: &mut Application, event_: Result<Event>) -> Result<()> {
    match event_? {
        Event::Key(key_event) => match key_event.code {
            event::KeyCode::Char('q') => app.quit(),
            event::KeyCode::Esc => app.quit(),
            event::KeyCode::Enter => { app.authorization_flow_spotify() },
          
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
