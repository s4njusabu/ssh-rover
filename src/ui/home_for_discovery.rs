// Creates the home menu and banner of SSH ROVER

use crate::state::State;
use crate::ui::panes::{pane1, pane2};
use ratatui::text::Line;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Paragraph},
};

const BANNER: &str = include_str!("../../assets/banner.txt");

// 1
// The draw function that calls both banner and content draw functions (this is the main draw function)
pub fn draw(frame: &mut Frame, area: Rect, state: &State) -> Rect {
    let [banner_area, content_area] =
        Layout::vertical([Constraint::Length(12), Constraint::Min(0)]).areas(area);

    draw_banner(frame, banner_area, state);

    // draw_content function code
    // container border below the banner; outermost border (the border that holds the panes)
    let colors = state.theme.colors();
    let content_area = content_area.inner(Margin {
        horizontal: 1,
        vertical: 0,
    });

    let content_block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(colors.accent));

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

    pane1::draw(frame, menu_area, state);
    pane2::layout::draw(frame, actions_area, state);

    output_area
}

// 2
fn draw_banner(frame: &mut Frame, area: Rect, state: &State) {
    let colors = state.theme.colors();
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
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(banner_block.clone(), banner_area);

    let inner_banner = banner_block.inner(banner_area).inner(Margin {
        horizontal: 0,
        vertical: 1,
    });
    let [banner_text_area] = Layout::horizontal([Constraint::Length(banner_width)])
        .flex(Flex::Center)
        .areas(inner_banner);

    frame.render_widget(
        Paragraph::new(BANNER).style(Style::default().fg(colors.banner)),
        banner_text_area,
    );
}

pub fn draw_pane_3(frame: &mut Frame, area: Rect, state: &State) -> Rect {
    let colors = state.theme.colors();

    let pane = Block::bordered()
        .border_type(BorderType::Thick)
        .title(
            Line::from(" 3 ").style(
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .border_style(Style::default().fg(colors.accent));

    frame.render_widget(pane.clone(), area);

    pane.inner(area).inner(Margin {
        horizontal: 2,
        vertical: 1,
    })
}
