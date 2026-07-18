// State of the App
use crate::ui::{panes::pane2, themes::Theme};

#[derive(PartialEq)]
pub enum Selected {
    Discovery(usize),
    Dependencies(usize),
    Themes(usize),
    About(usize),
    Exit,
    Back,
}

pub struct State {
    pub theme: Theme,

    // Pane 1
    pub in_pane1: bool,
    pub hovered: usize,
    pub selected: Selected,

    // Pane 2
    pub in_pane2: bool,
    pub pane2_hovered: Option<usize>,
    pub pane2_selected: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            theme: Theme::Default,
            in_pane1: true,
            hovered: 0,
            selected: Selected::Discovery(pane2::discovery::ITEM_COUNT),

            in_pane2: false,
            pane2_hovered: None,
            pane2_selected: 0,
        }
    }
}
