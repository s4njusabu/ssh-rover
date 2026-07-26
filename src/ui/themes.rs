use ratatui::style::Color;

pub struct ThemeColors {
    pub accent: Color,
    pub background: Color,
    pub banner: Color,
    pub text: Color,
    pub active: Color,

    pub ssh_text: Color,
    pub rover_text: Color,
}

pub enum Theme {
    Default,
    Red,
    Blue,
    Green,
    Yellow,
    Magenta,
    Gray,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::Default => ThemeColors {
                accent: Color::LightBlue,
                background: Color::Black,
                banner: Color::White,

                text: Color::White,
                active: Color::LightBlue,

                ssh_text: Color::White,
                rover_text: Color::LightBlue,
            },
            Theme::Red => ThemeColors {
                accent: Color::LightRed,
                background: Color::Black,
                banner: Color::LightRed,

                text: Color::Red,
                active: Color::LightRed,

                ssh_text: Color::LightRed,
                rover_text: Color::LightRed,
            },
            Theme::Blue => ThemeColors {
                accent: Color::LightBlue,
                background: Color::Black,
                banner: Color::LightBlue,

                text: Color::Blue,
                active: Color::LightBlue,

                ssh_text: Color::LightBlue,
                rover_text: Color::LightBlue,
            },

            Theme::Green => ThemeColors {
                accent: Color::LightGreen,
                background: Color::Black,
                banner: Color::LightGreen,

                text: Color::Green,
                active: Color::LightGreen,

                ssh_text: Color::LightGreen,
                rover_text: Color::LightGreen,
            },
            Theme::Yellow => ThemeColors {
                accent: Color::LightYellow,
                background: Color::Black,
                banner: Color::LightYellow,

                text: Color::Yellow,
                active: Color::LightYellow,

                ssh_text: Color::LightYellow,
                rover_text: Color::LightYellow,
            },
            Theme::Magenta => ThemeColors {
                accent: Color::LightMagenta,
                background: Color::Black,
                banner: Color::LightMagenta,

                text: Color::Magenta,
                active: Color::LightMagenta,

                ssh_text: Color::LightMagenta,
                rover_text: Color::LightMagenta,
            },
            Theme::Gray => ThemeColors {
                accent: Color::DarkGray,
                background: Color::Black,
                banner: Color::Gray,

                text: Color::DarkGray,
                active: Color::White,

                ssh_text: Color::Gray,
                rover_text: Color::Gray,
            },
        }
    }
}
