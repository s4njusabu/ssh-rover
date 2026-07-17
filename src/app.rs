// State of the App
use crate::ui::themes::Theme;

#[derive(PartialEq)]
pub enum Selected {
    Discovery,
    Dependencies,
    Themes,
    About,
    Exit,
}

pub struct App {
    pub theme: Theme,
    pub hovered: usize,
    pub selected: Selected,
}

impl App {
    pub fn new() -> Self {
        App {
            theme: Theme::Default,
            hovered: 0,
            selected: Selected::Discovery,
        }
    }
}
