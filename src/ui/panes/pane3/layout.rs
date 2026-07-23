use crate::{
    state::{Pane1, State},
    ui::panes::pane3::{dependencies, project, themes},
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

    match state.pane1_selected {
        Pane1::Discovery(_) => {}
        Pane1::Dependencies(_) => match state.pane2_hovered {
            Some(0) => dependencies::check_status::draw(frame, inner, state),
            Some(1) => dependencies::install_nmap::draw(frame, inner, state),
            Some(2) => dependencies::install_openssh::draw(frame, inner, state),
            Some(3) => dependencies::install_both::draw(frame, inner, state),
            _ => {}
        },
        Pane1::Themes(_) => match state.pane2_hovered {
            Some(0) => themes::default::draw(frame, inner, state),
            Some(1) => themes::red::draw(frame, inner, state),
            Some(2) => themes::blue::draw(frame, inner, state),
            Some(3) => themes::green::draw(frame, inner, state),
            Some(4) => themes::yellow::draw(frame, inner, state),
            Some(5) => themes::magenta::draw(frame, inner, state),
            Some(6) => themes::gray::draw(frame, inner, state),
            _ => {}
        },
        Pane1::Project(_) => {
            if let Some(0) = state.pane2_hovered {
                project::draw(frame, inner, state)
            }
        }
        Pane1::Exit => {}
    }
}
