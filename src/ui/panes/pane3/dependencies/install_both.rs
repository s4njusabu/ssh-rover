use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    services,
    state::{Pane3InstallState, State},
};

pub fn draw(frame: &mut Frame, inner: Rect, state: &State) {
    let colors = state.theme.colors();

    let [info, footer] = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);
    let [title, distro, package, command] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(info);

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

    if !state.nmap_installed || !state.openssh_installed {
        let footer_text = match state.pane3_nmap_install_state {
            Pane3InstallState::Ready => "PRESS ENTER TO INSTALL",
            Pane3InstallState::Password => "❯ ENTER SUDO PASSWORD:",
            Pane3InstallState::Installing => "INSTALLING...",
            Pane3InstallState::Success => "✓ INSTALLED SUCCESSFULLY",
            Pane3InstallState::Failed => "✗ INSTALLATION FAILED",
        };

        if state.pane3_both_install_state == Pane3InstallState::Failed {
            frame.render_widget(
                Paragraph::new(footer_text).style(
                    Style::default()
                        .fg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                ),
                footer,
            );
        } else {
            frame.render_widget(
                Paragraph::new(footer_text).style(
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                footer,
            );
        }
    } else {
        if state.pane2_selected == 3 {
            frame.render_widget(
                Paragraph::new("✓ BOTH ARE INSTALLED").style(
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                footer,
            );
        } else {
            frame.render_widget(
                Paragraph::new("").style(
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                footer,
            );
        }
    }
}
