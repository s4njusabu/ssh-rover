use crate::state::State;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

pub const ITEM_COUNT: usize = 4 + 1;

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [status, nmap, openssh, both, back] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    let hovered = state.pane2_hovered;

    if hovered == Some(0) {
        frame.render_widget(
            Paragraph::new("❯ CHECK STATUS").style(
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
            status,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  CHECK STATUS").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            status,
        );
    }

    if hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("❯ NMAP").style(
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
            nmap,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  NMAP").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            nmap,
        );
    }

    if hovered == Some(2) {
        frame.render_widget(
            Paragraph::new("❯ OPENSSH").style(
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
            openssh,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  OPENSSH").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            openssh,
        );
    }

    if hovered == Some(3) {
        frame.render_widget(
            Paragraph::new("❯ NMAP + OPENSSH").style(
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
            both,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  NMAP + OPENSSH").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            both,
        );
    }

    if state.in_pane2 {
        if hovered == Some(4) {
            frame.render_widget(
                Paragraph::new("❯ BACK").style(
                    Style::default()
                        .fg(colors.active)
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
