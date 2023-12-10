use ratatui::{Frame, widgets::{Block, Borders, Paragraph}, layout::Rect};

use crate::application::Application;

pub fn render(app: &mut Application, frame : &mut Frame) { 
    let size = frame.size();
    let mut y = 0;

    frame.render_widget(
        Paragraph::new("Spotify"), 
    Rect::new(0, 500, size.width, size.height)
    );
}
