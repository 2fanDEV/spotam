use color_eyre::owo_colors::{
    self,
    colors::{css::WhiteSmoke, Red},
    OwoColorize,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize, Styled},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::application::Application;

pub fn render_streaming_service_selection_screen(services: Vec<&str>, frame: &mut Frame, state : [u8;2]) {
    let mut i = 0;
    let mut y = state[0];
    let text = services
        .iter()
        .map(|x| {
            let mut line = Line::from(Span::styled(*x, {
                if i == y {
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::White)
                        .rapid_blink()
                } else {
                    Style::default().add_modifier(Modifier::BOLD)
                }
            }));
                // line.spans.get_mut(0).unwrap().set_style(Style::default()
                // .add_modifier(Modifier::BOLD)
                // .bg(Color::White)
                // .rapid_blink());
            i = i + 1;
            line
        })
        .collect::<Vec<Line>>();

    let x = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Streaming Services"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(
        x,
        Rect::new(0, 0, frame.size().width / 3, frame.size().height / 3),
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
