use crate::state::State;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

pub const ITEM_COUNT: usize = 3 + 1;

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [current, cidr, manual, back] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    let hovered = state.pane2_hovered;

    if hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("❯ SCAN CURRENT NETWORK").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            current,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  SCAN CURRENT NETWORK").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            current,
        );
    }

    if hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("❯ SCAN CIDR RANGE").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            cidr,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  SCAN CIDR RANGE").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            cidr,
        );
    }

    if hovered == Some(2) {
        frame.render_widget(
            Paragraph::new("❯ MANUAL CONNECT").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            manual,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  MANUAL CONNECT").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            manual,
        );
    }

    if state.in_pane2 {
        if hovered == Some(3) {
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
