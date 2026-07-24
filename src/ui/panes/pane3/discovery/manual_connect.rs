#![allow(unused)]

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::{services::discovery::network, state::State};

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

    let interface_name = if let Some(s1) = network::get_interface() {
        s1
    } else {
        "UNKNOWN".to_string()
    };

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "INTERFACE"),
                Style::default().fg(colors.text),
            )
            .add_modifier(Modifier::BOLD),
            Span::styled(
                &interface_name,
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        interface,
    );

    let network_range = if let Some(s2) = network::get_interface_cidr(&interface_name) {
        s2
    } else {
        "UNKNOWN".to_string()
    };

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

    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.activity_pane));

    frame.render_widget(pane, output);
}
