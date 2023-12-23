
use ratatui::{
    layout::{Alignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::application::Application;
use crate::ui_layout::*;

pub fn render(app: &mut Application, frame: &mut Frame) {
    let x = ui_layout_selection_login_application(&app, frame.size());
    frame.render_widget(x.widget, x.area)
    // let layout = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints(vec![Constraint::Percentage(15),
    //                      Constraint::Percentage(85)])
    //     .split(frame.size());
}
