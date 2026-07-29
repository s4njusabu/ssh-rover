use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::state::State;

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [title, github, cargo] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new("INSTALLATION METHODS").style(
            Style::default()
                .fg(colors.active)
                .add_modifier(Modifier::BOLD),
        ),
        title,
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<10}", "GITHUB"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "https://github.com/s4njusabu/ssh-rover",
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        github,
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<10}", "CARGO"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "cargo install ssh-rover",
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        cargo,
    );
}
