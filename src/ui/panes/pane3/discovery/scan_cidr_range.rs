use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::{state::State, ui::panes::pane3::discovery::prompt};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let [label, input_area, output, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);

    let [input, _] =
        Layout::horizontal([Constraint::Length(42), Constraint::Min(0)]).areas(input_area);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                format!("{:<15}", "CIDR RANGE"),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Example: 192.168.1.0/24",
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
        ])),
        label,
    );

    let cidr_line = if state.entering_cidr {
        Line::from(Span::styled(
            format!(" {}|", state.cidr_range),
            Style::default()
                .fg(colors.active)
                .add_modifier(Modifier::BOLD),
        ))
    } else {
        Line::from(Span::styled(
            format!(" {}", state.cidr_range),
            Style::default()
                .fg(colors.active)
                .add_modifier(Modifier::BOLD),
        ))
    };

    frame.render_widget(
        Paragraph::new(cidr_line).block(
            Block::bordered()
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(colors.active)),
        ),
        input,
    );
    if state.entering_cidr {
    } else if state.scanned_ips.is_empty() {
        frame.render_widget(
            Paragraph::new("NO HOSTS FOUND").style(
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            ),
            output,
        );
    } else if state.entering_username {
        prompt::username_prompt(frame, output, state);
    } else {
        frame.render_widget(
            Paragraph::new(
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
            ),
            output,
        );
    }

    let footer_text = if state.entering_cidr {
        "[Enter] SCAN    [Esc] BACK"
    } else if state.entering_username {
        ""
    } else if state.scanned_ips.is_empty() {
        "[Esc] BACK"
    } else {
        "[↑↓] SELECT    [Enter] CONNECT    [Esc] BACK"
    };

    frame.render_widget(
        Paragraph::new(Span::styled(
            footer_text,
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        )),
        footer,
    );
}
