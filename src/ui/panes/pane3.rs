use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &App) {
    let colors = state.theme.colors();
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Thick)
            .title(
                Line::from(" 3 ").style(
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .border_style(Style::default().fg(colors.accent)),
        area,
    );
}
