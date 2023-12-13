use color_eyre::owo_colors::{BgColorDisplay, OwoColorize};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::application::Application;

pub fn render_streaming_service_selection_screen(services: Vec<&str>, frame: &mut Frame) {
    let mut list: Vec<ListItem> = Vec::<ListItem>::new();

    services.iter().for_each(|&srvc| {
        list.push(ListItem::new(
            Line::from(Span::styled(srvc, Style::default())).alignment(Alignment::Center),
        ))
    });

    // let x = List::new(list).block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .title("Streaming Services")
    //         .title_alignment(ratatui::layout::Alignment::Center),
    //     );

    frame.render_widget(
        x,
        Rect::new(0, 0, frame.size().width / 4, frame.size().height / 3),
    );
}

pub fn render(app: &mut Application, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());
    let size = frame.size();
    frame.render_widget(
        Block::new().borders(Borders::ALL).title("Services"),
        layout[1],
    );
}
