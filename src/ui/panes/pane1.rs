pub mod layout;
pub mod menu;

use crate::state::State;
use ratatui::{Frame, layout::Rect};

pub fn draw(frame: &mut Frame, area: Rect, state: &State) {
    let inner = layout::draw(frame, area, state);
    menu::draw(frame, inner, state);
}
