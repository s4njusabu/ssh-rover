// Creates the home menu and banner of MuxSSH

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::Style,
    widgets::Paragraph,
};

const BANNER: &str = include_str!("../../assets/banner.txt");

pub fn draw(frame: &mut Frame) {
    let banner_width = BANNER
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;

    let [_, top_area, _] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Min(0),
    ])
    .areas(frame.area());

    let [banner_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(top_area);

    frame.render_widget(
        Paragraph::new(BANNER).style(Style::default().fg(ratatui::style::Color::White)),
        banner_area,
    );
}
