use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

use crate::state::State;

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();

    let area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let hint = if state.entering_manual_username {
        "[Enter] CONNECT    [Esc] BACK"
    } else {
        "[Enter] NEXT    [Esc] BACK"
    };

    let [ip_label, ip_box, username_label, username_box, _, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new("IP ADDRESS").style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        ip_label,
    );

    let [ip_box, _] =
        Layout::horizontal([Constraint::Length(42), Constraint::Min(0)]).areas(ip_box);

    frame.render_widget(
        Paragraph::new(Line::from(vec![if state.entering_manual_username {
            Span::styled(
                format!(" {}", state.manual_ip),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                format!(" {}|", state.manual_ip),
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            )
        }]))
        .block(
            Block::bordered()
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(colors.active)),
        ),
        ip_box,
    );

    frame.render_widget(
        Paragraph::new("USERNAME").style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        username_label,
    );

    let [username_box, _] =
        Layout::horizontal([Constraint::Length(42), Constraint::Min(0)]).areas(username_box);

    frame.render_widget(
        Paragraph::new(Line::from(vec![if state.entering_manual_username {
            Span::styled(
                format!(" {}|", state.manual_username),
                Style::default()
                    .fg(colors.active)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                "",
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            )
        }]))
        .block(
            Block::bordered()
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(colors.active)),
        ),
        username_box,
    );

    frame.render_widget(
        Paragraph::new(hint).style(
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        footer,
    );
}
