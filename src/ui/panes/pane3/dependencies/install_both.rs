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

    let [title, distro, package, command] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(inner);

    // Title
    frame.render_widget(
        Paragraph::new("INSTALLS BOTH NMAP AND OPENSSH").style(
            Style::default()
                .fg(colors.accent)
                .add_modifier(Modifier::BOLD),
        ),
        title,
    );

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
    let nmap_name = super::package_manager::nmap_package_name(package_manager);
    let openssh_name = super::package_manager::openssh_package_name(package_manager);
    let package_names = format!("{nmap_name} {openssh_name}").replace(" ", ", ");

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "PACKAGE"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                package_names,
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        package,
    );

    // Command
    let command_to_install_both = super::package_manager::install_nmap_and_openssh(
        package_manager,
        format!("{nmap_name} {openssh_name}").as_str(),
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "COMMAND"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                command_to_install_both,
                Style::default()
                    .fg(colors.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        command,
    );
}
