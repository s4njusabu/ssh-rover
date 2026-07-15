// State of the App
use crate::ui::themes::Theme;

pub enum View {
    Home,
    QuickConnect,
    SavedHosts,
    Dependencies,
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
            theme: Theme::Yellow,
            selected: 0,
            view: View::Home,
        }
    }
}
