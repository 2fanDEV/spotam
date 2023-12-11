
use crate::{event::Event, application::Application};
use color_eyre::Result;
use crossterm::event::{self, KeyEvent, KeyCode};

pub fn update(app : &mut Application, event_: Result<Event>) -> Result<()> {
    match event_? { 
        Event::Key(key_event) => { 
            match key_event.code {
                event::KeyCode::Char('q') => app.quit(),
                _ => {}
            }
        },
        _ => {}
    }
    Ok(())
}