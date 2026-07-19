use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{services, state::State};

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [title, status, distro, package, command] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    // Title
    frame.render_widget(
        Paragraph::new("OPENSSH IS USED TO SECURELY CONNECT TO REMOTE SYSTEMS").style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
        title,
    );

    // Status
    let is_openssh_installed = crate::services::dependencies::openssh_installed();
    if is_openssh_installed {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<15}", "STATUS"),
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
            status,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<15}", "STATUS"),
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
            status,
        );
    }

    // Distro
    let hostname = services::dependencies::get_os_id().unwrap_or_else(|_| "NOT FOUND".to_string());

    if hostname != "NOT FOUND" {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<15}", "DISTRIBUTION"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if hostname == "arch" {
                        "ARCH LINUX".to_string()
                    } else {
                        hostname.to_uppercase()
                    },
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            distro,
        );
    } else {
        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{:<15}", "DISTRIBUTION"),
                    Style::default()
                        .fg(colors.text)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "UNKNOWN DISTRIBUTION",
                    Style::default()
                        .fg(ratatui::style::Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
            ])),
            distro,
        );
    }

    // Package
    let package_manager = super::package_manager::package_manager(&hostname);
    let openssh_name =
        super::package_manager::openssh_package_name(package_manager).replace(" ", ", ");

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "PACKAGE"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                openssh_name,
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        package,
    );

    // Command
    let command_to_install_openssh =
        super::package_manager::openssh_package_install(package_manager);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "COMMAND"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                command_to_install_openssh,
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        command,
    );
}
