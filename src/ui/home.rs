// Creates the home menu and banner of MuxSSH

use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};

const BANNER: &str = include_str!("../../assets/banner.txt");
pub const MENU_ITEMS: [&str; 6] = [
    "QUICK CONNECT",
    "SAVED HOSTS",
    "DEPENDENCIES",
    "THEMES",
    "ABOUT",
    "EXIT",
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
fn draw_content(frame: &mut Frame, area: Rect, _state: &App) {
    // 1
    // container border below the banner; outermost border (the border that holds the panes)
    let content_area = area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let content_block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::LightBlue));

    frame.render_widget(content_block.clone(), content_area);

    let inner = content_block.inner(content_area);

    //2
    // Panes layout
    let [_, menu_area, actions_area, output_area, _] = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Length(30),
        Constraint::Length(35),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(inner);

    // Panes

    draw_pane_1(frame, menu_area, _state);
    draw_pane_2(frame, actions_area, _state);
    draw_pane_3(frame, output_area, _state);

    // 3
    // Options menu ()

    // 4
    // Sub menu (Child cell 2)

    // 5
    // Output menu (Child cell 3)
}

// Panes
fn draw_pane_1(frame: &mut Frame, area: Rect, state: &App) {
    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .title(
            Line::from(" 1 ").style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_style(Style::default().fg(Color::LightBlue));

    frame.render_widget(pane.clone(), area);

    let inner = pane.inner(area);
    let inner = inner.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let [quick, saved, dependencies, themes, about, exit] = Layout::vertical([
        Constraint::Length(2),
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
            Paragraph::new(MENU_ITEMS[0]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            quick,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[0]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            quick,
        );
    }

    if state.selected == 1 {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[1]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            saved,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[1]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            saved,
        );
    }
    if state.selected == 2 {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[2]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[2]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            dependencies,
        );
    }

    if state.selected == 3 {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[3]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            themes,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[3]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            themes,
        );
    }

    if state.selected == 4 {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[4]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[4]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            about,
        );
    }

    if state.selected == 5 {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[5]).style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    } else {
        frame.render_widget(
            Paragraph::new(MENU_ITEMS[5]).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            exit,
        );
    }
}

fn draw_pane_2(frame: &mut Frame, area: Rect, _state: &App) {
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Thick)
            .title(
                Line::from(" 2 ").style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .border_style(Style::default().fg(Color::LightBlue)),
        area,
    );
}

fn draw_pane_3(frame: &mut Frame, area: Rect, _state: &App) {
    frame.render_widget(
        Block::bordered()
            .border_type(BorderType::Thick)
            .title(
                Line::from(" 3 ").style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            )
            .border_style(Style::default().fg(Color::LightBlue)),
        area,
    );
}
