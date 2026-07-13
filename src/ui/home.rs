// Creates the home menu and banner of MuxSSH

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

const BANNER: &str = include_str!("../../assets/banner.txt");
pub const MENU_ITEMS: [&str; 5] = [
    "Quick Connect",
    "Saved Hosts",
    "Dependencies",
    "About",
    "Exit",
];

pub fn draw(frame: &mut Frame, area: Rect, state: &App) {
    let banner_width = BANNER
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    let [_, top_area, menu_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Min(0),
    ])
    .areas(area);

    let [banner_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(top_area);

    // Banner
    frame.render_widget(
        Paragraph::new(BANNER).style(Style::default().fg(ratatui::style::Color::White)),
        banner_area,
    );

    let [_, menu_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(menu_area);

    let [quick, saved, dependencies, about, exit] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ])
    .areas(menu_area);

    if state.selected == 0 {
        frame.render_widget(
            Paragraph::new("Quick Connect").centered().style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            quick,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Quick Connect")
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
            quick,
        );
    }

    if state.selected == 1 {
        frame.render_widget(
            Paragraph::new("Saved Hosts").centered().style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            saved,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Saved Hosts")
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
            saved,
        );
    }
    if state.selected == 2 {
        frame.render_widget(
            Paragraph::new("Dependencies").centered().style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Dependencies")
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
            dependencies,
        );
    }

    if state.selected == 3 {
        frame.render_widget(
            Paragraph::new("About").centered().style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    } else {
        frame.render_widget(
            Paragraph::new("About")
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
            about,
        );
    }

    if state.selected == 4 {
        frame.render_widget(
            Paragraph::new("Exit").centered().style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    } else {
        frame.render_widget(
            Paragraph::new("Exit")
                .style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .centered(),
            exit,
        );
    }
}
