use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};

pub fn draw(frame: &mut Frame, area: Rect, state: &App) {
    let colors = state.theme.colors();
    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .title(
            Line::from(" 1 ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(pane.clone(), area);

    let inner = pane.inner(area);
    let inner = inner.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let [discovery, dependencies, themes, about, exit] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .spacing(1)
    .areas(inner);

    if state.selected == 0 {
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

    if state.selected == 1 {
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

    if state.selected == 2 {
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

    if state.selected == 3 {
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

    if state.selected == 4 {
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
