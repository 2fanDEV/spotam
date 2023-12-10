use color_eyre::owo_colors::OwoColorize;
use ratatui::{Frame, widgets::{Block, Borders, Paragraph}, layout::Rect};

use crate::application::Application;

pub fn render(app: &mut Application, frame : &mut Frame, x: u16, y: u16) { 
    let size = frame.size();
    frame.render_widget(
        Paragraph::new("Spotify"), 
    Rect::new(x, y, size.width, size.height)
    );
}

pub fn render2(app: &str, frame : &mut Frame, x: u16, y: u16) { 
    let size = frame.size();
    frame.render_widget(
        Paragraph::new(app), 
    Rect::new(x, y, size.width, size.height)
    );
}
