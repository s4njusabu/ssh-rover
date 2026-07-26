#![allow(unused)]

use std::{sync::mpsc, thread};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::{
    services::{
        self,
        discovery::{network, scan_cidr_range},
    },
    state::State,
    ui::panes::pane3::discovery::prompt,
};
pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [interface, network, output] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Min(0),
    ])
    .areas(area);

    let interface_name = network::get_interface().unwrap_or_else(|| "UNKNOWN".to_string());

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "INTERFACE"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &interface_name,
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        interface,
    );

    let network_range =
        network::get_interface_cidr(&interface_name).unwrap_or_else(|| "UNKNOWN".to_string());

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "NETWORK"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &network_range,
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        network,
    );

    if state.pane2_hovered == Some(0) {
        if state.scanned_ips.is_empty() {
            frame.render_widget(
                Paragraph::new("PRESS ENTER TO SCAN\n\nTHE SYSTEM MAY LAG DURING THE SCAN").style(
                    Style::default()
                        .fg(colors.active)
                        .add_modifier(Modifier::BOLD),
                ),
                output,
            );
        } else if state.entering_username {
            prompt::username_prompt(frame, output, state);
        } else {
            let widget = Paragraph::new(
                state
                    .scanned_ips
                    .iter()
                    .enumerate()
                    .map(|(i, ip)| {
                        if i == state.selected_ip {
                            Line::from(Span::styled(
                                format!("> {}", ip),
                                Style::default()
                                    .fg(colors.text)
                                    .add_modifier(Modifier::BOLD),
                            ))
                        } else {
                            Line::from(Span::styled(
                                format!("  {}", ip),
                                Style::default()
                                    .fg(colors.active)
                                    .add_modifier(Modifier::BOLD),
                            ))
                        }
                    })
                    .collect::<Vec<_>>(),
            );

            frame.render_widget(widget, output);
        }
    }
}
