use crate::state::State;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

pub const ITEM_COUNT: usize = 5;

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [default, red, green, yellow, blue] = Layout::vertical([
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
            Paragraph::new("❯ DEFAULT").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            default,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  DEFAULT").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            default,
        );
    }

    if hovered == Some(1) {
        frame.render_widget(
            Paragraph::new("❯ RED").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            red,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  RED").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            red,
        );
    }

    if hovered == Some(2) {
        frame.render_widget(
            Paragraph::new("❯ GREEN").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            green,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  GREEN").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            green,
        );
    }

    if hovered == Some(3) {
        frame.render_widget(
            Paragraph::new("❯ YELLOW").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            yellow,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  YELLOW").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            yellow,
        );
    }

    if hovered == Some(4) {
        frame.render_widget(
            Paragraph::new("❯ BLUE").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            blue,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  BLUE").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            blue,
        );
    }
}
