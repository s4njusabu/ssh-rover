use crate::state::State;

use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) -> Rect {
    let colors = state.theme.colors();
    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .title(
            Line::from(" 1 ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(pane.clone(), area);

    let inner = pane.inner(area);

    inner.inner(Margin {
        horizontal: 2,
        vertical: 1,
    })
}
