use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::state::State;

pub fn username_prompt(frame: &mut Frame, area: Rect, state: &State) {
    if !state.entering_username {
        return;
    }

    let colors = state.theme.colors();

    let prompt = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("> {} : ", state.scanned_ips[state.selected_ip]),
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &state.username,
            Style::default()
                .fg(colors.active)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    frame.render_widget(prompt, area);
}
