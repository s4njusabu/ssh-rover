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

#[allow(unused)]
pub struct App {
    pub theme: Theme,

    // Pane 1
    pub hovered: usize,
    pub selected: Selected,

    // Pane 2
    pub dependency_hovered: usize,
    pub dependency_selected: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            theme: Theme::Default,
            hovered: 0,
            selected: Selected::Discovery,

            dependency_hovered: 0,
            dependency_selected: 0,
        }
    }
}
