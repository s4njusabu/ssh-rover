pub mod layout;
pub mod menu;

use crate::app::App;
use ratatui::{Frame, layout::Rect};

pub fn draw(frame: &mut Frame, area: Rect, state: &App) {
    let inner = layout::draw(frame, area, state);
    menu::draw(frame, inner, state);
}
