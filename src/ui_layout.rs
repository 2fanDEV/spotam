use ratatui::{
    layout::{Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::application::Application;

pub struct UIElement<W>
where
    W: Widget,
{
    pub layout: Layout,
    pub widget: W,
    pub area: Rect,
}

impl<W> UIElement<W>
where
    W: Widget,
{
    pub fn new(layout: Layout, widget: W, area: Rect) -> Self {
        Self {
            layout,
            widget,
            area,
        }
    }
}

pub fn ui_layout_selection_login_application(
    application: &Application,
    frame_size: Rect,
) -> UIElement<Paragraph> {
    let mut i = 0;
    let layout = Layout::default();
    let not_logged_in_services = application
        .streaming_services
        .iter()
        .filter(|&service| !service.is_user_logged_in())
        .map(|service| service.get_name())
        .collect::<Vec<&str>>();

    let selections = not_logged_in_services
        .iter()
        .map(|x| {
            let mut line = Line::from(Span::styled(*x, {
                if i == 0 {
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::White)
                } else {
                    Style::default().add_modifier(Modifier::REVERSED)
                }
            }));
            line
        })
        .collect::<Vec<Line>>();

    let mut paragraph = Paragraph::new(selections).alignment(ratatui::layout::Alignment::Center);
    paragraph = add_block_to_widget(&mut paragraph, "Streaming Services");

    UIElement::new(layout, paragraph, Rect::new(0,0, frame_size.width/4, frame_size.height/4))
}

pub fn add_block_to_widget<'a>(widget: &mut Paragraph<'a>, headline: &'a str) -> Paragraph<'a> {
    widget.clone().block(Block::default().borders(Borders::ALL).title(headline))
}

//check if even necessary
pub fn ui_layout_login_screen(application: &Application, frame_size: Rect) {
    let layout = Layout::default().split(frame_size);
}
