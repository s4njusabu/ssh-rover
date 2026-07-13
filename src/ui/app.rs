// Creates the border and title of MuxSSH

use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame) {
    let block = Block::bordered()
        .title(
            Line::from(" MuxSSH ").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_type(BorderType::Thick)
        .style(Style::default().fg(Color::LightBlue));

    frame.render_widget(block, frame.area());
}
