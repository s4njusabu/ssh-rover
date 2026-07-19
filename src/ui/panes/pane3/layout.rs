use crate::{state::State, ui::panes::pane3::dependencies};
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
            Line::from(" 3 ").style(
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

    match state.pane2_hovered {
        Some(0) => dependencies::check_status::draw(frame, inner, state),
        Some(1) => dependencies::install_nmap::draw(frame, inner, state),
        Some(2) => dependencies::install_openssh::draw(frame, inner, state),
        Some(3) => dependencies::install_both::draw(frame, inner, state),
        _ => {}
    }
}
