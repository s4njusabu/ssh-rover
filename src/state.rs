// State of the App
use crate::{
    services,
    ui::{panes::pane2, themes::Theme},
};

#[derive(PartialEq)]
pub enum Pane1 {
    Discovery(usize),
    Dependencies(usize),
    Themes(usize),
    Project(usize),
    Exit,
}

pub struct State {
    pub theme: Theme,

    // Pane 1
    pub in_pane1: bool,
    pub hovered: usize,
    pub pane1_selected: Pane1,

    // Pane 2
    pub in_pane2: bool,
    pub pane2_hovered: Option<usize>,
    pub pane2_selected: usize,

    // Pane 3
    pub in_pane3: bool,
    pub pane3_hovered: Option<usize>,
    pub pane3_selected: usize,

    // Dependencies
    pub nmap_installed: bool,
    pub openssh_installed: bool,
}

impl State {
    pub fn new() -> Self {
        let nmap_installed = services::dependencies::nmap_installed();
        let openssh_installed = services::dependencies::openssh_installed();

        State {
            theme: Theme::Default,
            in_pane1: true,
            hovered: 0,
            pane1_selected: Pane1::Discovery(pane2::discovery::ITEM_COUNT),

            in_pane2: false,
            pane2_hovered: None,
            pane2_selected: 0,

            in_pane3: false,
            pane3_hovered: None,
            pane3_selected: 0,

            nmap_installed,
            openssh_installed,
        }
    }

    #[allow(unused)]
    pub fn refresh_dependencies(&mut self) {
        self.nmap_installed = services::dependencies::nmap_installed();
        self.openssh_installed = services::dependencies::openssh_installed();
    }
}
