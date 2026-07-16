// State of the App
use crate::ui::themes::Theme;

#[derive(PartialEq)]
pub enum View {
    Discovery,
    Dependencies,
    Themes,
    About,
    Exit,
}

pub struct App {
    pub theme: Theme,
    pub selected: usize,
    pub view: View,
}

impl App {
    pub fn new() -> Self {
        App {
            theme: Theme::Default,
            selected: 0,
            view: View::Discovery,
        }
    }
}
