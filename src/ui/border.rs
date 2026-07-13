// Creates the border and title of MuxSSH

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame) -> Rect {
    let block = Block::bordered()
        .title(Line::from(vec![
            Span::styled(
                " Mux",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "SSH ",
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightBlue));

    let inner = block.inner(frame.area());

    frame.render_widget(block, frame.area());

    inner
}
