use crate::state::State;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

pub const ITEM_COUNT: usize = 2 + 1;

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [github, aur, back] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    let hovered = state.pane2_hovered;

    if hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("❯ GITHUB").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            github,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  GITHUB").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            github,
        );
    }

    if hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("❯ AUR").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            aur,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  AUR").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            aur,
        );
    }

    if state.in_pane2 == true {
        if hovered == Some(2) {
            frame.render_widget(
                Paragraph::new("❯ BACK").style(
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                back,
            );
        } else {
            frame.render_widget(
                Paragraph::new("  BACK").style(
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                back,
            );
        }
    }
}
