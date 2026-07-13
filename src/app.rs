// State of the App

pub enum View {
    Home,
    QuickConnect,
    SavedHosts,
    Dependencies,
    About,
    Exit,
}

pub struct App {
    pub selected: usize,
    pub view: View,
}

impl App {
    pub fn new() -> Self {
        App {
            selected: 0,
            view: View::Home,
        }
    }
}
