// Creates the border and title of MuxSSH
// Added custom color

use crate::state::State;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame, state: &State) -> Rect {
    let colors = state.theme.colors();

    let block = Block::bordered()
        .title(Line::from(vec![
            Span::styled(
                " Mux",
                Style::default()
                    .fg(colors.mux_text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "SSH ",
                Style::default()
                    .fg(colors.ssh_text)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

    let inner = block.inner(frame.area());

    frame.render_widget(block, frame.area());

    inner
}
