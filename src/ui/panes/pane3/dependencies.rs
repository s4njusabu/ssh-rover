use std::vec;

use crate::state::State;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn draw_check_status(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [title, nmap, openssh] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new("DEPENDENCY STATUS").style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
        title,
    );

    let is_nmap_installed = crate::services::dependencies::nmap_installed();
    let is_openssh_installed = crate::services::dependencies::openssh_installed();

    if is_nmap_installed {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<10}", "NMAP"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "INSTALLED",
                    Style::default()
                        .fg(ratatui::style::Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            nmap,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<10}", "NMAP"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "NOT INSTALLED",
                    Style::default()
                        .fg(ratatui::style::Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            nmap,
        );
    }

    if is_openssh_installed {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<10}", "OPENSSH"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "INSTALLED",
                    Style::default()
                        .fg(ratatui::style::Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            openssh,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<10}", "OPENSSH"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "NOT INSTALLED",
                    Style::default()
                        .fg(ratatui::style::Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            openssh,
        );
    }
}

pub fn draw_install_nmap(frame: &mut Frame, inner: Rect, state: &State) {

}