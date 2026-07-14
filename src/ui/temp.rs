use crate::app::App;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

// 4
fn draw_content(frame: &mut Frame, area: Rect, state: &App) {
    let [line_area, content_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);

    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Thick)
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::LightBlue)),
        line_area,
    );

    let [menu_area, divider_area, preview_area] = Layout::horizontal([
        Constraint::Length(24),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .areas(content_area);

    frame.render_widget(
        Block::new()
            .borders(Borders::LEFT)
            .border_style(Style::default().fg(Color::LightBlue)),
        divider_area,
    );

    let menu_area = menu_area.inner(Margin {
        horizontal: 3,
        vertical: 1,
    });

    let [quick, saved, dependencies, about, exit] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
    ])
    .areas(menu_area);

    if state.selected == 0 {
        frame.render_widget(
            Paragraph::new("QUICK CONNECT").style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            quick,
        );
    } else {
        frame.render_widget(
            Paragraph::new("QUICK CONNECT").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            quick,
        );
    }

    if state.selected == 1 {
        frame.render_widget(
            Paragraph::new("SAVED HOSTS").style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            saved,
        );
    } else {
        frame.render_widget(
            Paragraph::new("SAVED HOSTS").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            saved,
        );
    }
    if state.selected == 2 {
        frame.render_widget(
            Paragraph::new("DEPENDENCIES").style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    } else {
        frame.render_widget(
            Paragraph::new("DEPENDENCIES").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    }

    if state.selected == 3 {
        frame.render_widget(
            Paragraph::new("ABOUT").style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    } else {
        frame.render_widget(
            Paragraph::new("ABOUT").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    }

    if state.selected == 4 {
        frame.render_widget(
            Paragraph::new("EXIT").style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    } else {
        frame.render_widget(
            Paragraph::new("EXIT").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    }
}
