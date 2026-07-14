// Creates the home menu and banner of MuxSSH

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

const BANNER: &str = include_str!("../../assets/banner.txt");
pub const MENU_ITEMS: [&str; 5] = [
    "Quick Connect",
    "Saved Hosts",
    "Dependencies",
    "About",
    "Exit",
];

// 1
// The draw function that calls both banner and content draw functions (this is the main draw function)
pub fn draw(frame: &mut Frame, area: Rect, state: &App) {
    let [banner_area, content_area] =
        Layout::vertical([Constraint::Length(12), Constraint::Min(0)]).areas(area);

    draw_banner(frame, banner_area);
    draw_content(frame, content_area, state);
}

// 2
fn draw_banner(frame: &mut Frame, area: Rect) {
    let banner_width = BANNER
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    let banner_area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let banner_block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightBlue));

    frame.render_widget(banner_block.clone(), banner_area);

    let inner_banner = banner_block.inner(banner_area).inner(Margin {
        horizontal: 0,
        vertical: 1,
    });
    let [banner_text_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(inner_banner);

    frame.render_widget(
        Paragraph::new(BANNER).style(Style::default().fg(Color::White)),
        banner_text_area,
    );
}

// 3
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
