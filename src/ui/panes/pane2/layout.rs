use crate::{
    state::State,
    ui::panes::pane2::{about, dependencies, discovery, themes},
};
use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .title(
            Line::from(" 2 ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(pane.clone(), area);

    let inner = pane.inner(area).inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    match state.hovered {
        0 => discovery::draw(frame, inner, state),
        1 => dependencies::draw(frame, inner, state),
        2 => themes::draw(frame, inner, state),
        3 => about::draw(frame, inner, state),
        _ => {}
    }
}
