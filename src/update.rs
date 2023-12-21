use crate::{application::Application, event::Event};
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};

pub fn update(app: &mut Application, event_: Result<Event>) -> Result<()> {
    match event_? {
        Event::Key(key_event) => match key_event.code {
            event::KeyCode::Char('q') => app.quit(),
            event::KeyCode::Esc => app.quit(),

            event::KeyCode::Up => app.set_state([
                if app.get_state()[0] == 0 {
                    app.get_state()[0]
                } else {
                    app.get_state()[0] - 1
                },
                app.get_state()[1],
            ]),
            event::KeyCode::Down => app.set_state([app.get_state()[0] + 1, app.get_state()[1]]),
            event::KeyCode::Right => app.set_state([app.get_state()[0], app.get_state()[1] + 1]),
            event::KeyCode::Left => app.set_state([app.get_state()[0], app.get_state()[1] - 1]),

            event::KeyCode::Enter => {}

            _ => {}
        },
        _ => {}
    }
    Ok(())
}
