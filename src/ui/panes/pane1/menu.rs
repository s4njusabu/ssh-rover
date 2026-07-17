use crate::app::App;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, inner: Rect, state: &App) {
    let colors = state.theme.colors();

    let [discovery, dependencies, themes, about, exit] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .spacing(1)
    .areas(inner);

    if state.hovered == 0 {
        frame.render_widget(
            Paragraph::new("❯ DISCOVERY").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            discovery,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  DISCOVERY").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            discovery,
        );
    }

    if state.hovered == 1 {
        frame.render_widget(
            Paragraph::new("❯ DEPENDENCIES").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  DEPENDENCIES").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    }

    if state.hovered == 2 {
        frame.render_widget(
            Paragraph::new("❯ THEMES").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            themes,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  THEMES").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            themes,
        );
    }

    if state.hovered == 3 {
        frame.render_widget(
            Paragraph::new("❯ ABOUT").style(
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  ABOUT").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    }

    if state.hovered == 4 {
        frame.render_widget(
            Paragraph::new("❯ EXIT").style(
                Style::default()
                    .fg(colors.danger)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    } else {
        frame.render_widget(
            Paragraph::new("  EXIT").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    }
}
