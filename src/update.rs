use crate::{application::Application, event::Event};
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};

pub fn update(app: &mut Application, event_: Result<Event>) -> Result<()> {
    match event_? {
        Event::Key(key_event) => match key_event.code {
            event::KeyCode::Char('q') => app.quit(),
            event::KeyCode::Esc => app.quit(),
            event::KeyCode::Enter => { 
                
                // println!("x = {}, y = {}", app.navigation.x, app.navigation.y);
                // println!("selected app: {}", app.streaming_services[app.navigation.y as usize].get_name())
            },
          
            event::KeyCode::Up => {
                app.navigation.move_up();
            }
            event::KeyCode::Down => {
                app.navigation.move_down();
            }

            _ => {}
        },
        _ => {}
    }
    Ok(())
}
