use ratatui::style::Color;

pub struct ThemeColors {
    pub accent: Color,
    pub background: Color,
    pub banner: Color,
    pub text: Color,
    pub active: Color,
    pub warning: Color,
    pub danger: Color,
    pub success: Color,

    pub ssh_text: Color,
    pub hopper_text: Color,
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
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::White,
                hopper_text: Color::LightBlue,
            },
            Theme::Red => ThemeColors {
                accent: Color::LightRed,
                background: Color::Black,
                banner: Color::LightRed,
                text: Color::Red,
                active: Color::LightRed,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,
                ssh_text: Color::LightRed,
                hopper_text: Color::LightRed,
            },
            Theme::Blue => ThemeColors {
                accent: Color::LightBlue,
                background: Color::Black,
                banner: Color::LightBlue,
                text: Color::Blue,
                active: Color::LightBlue,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::LightBlue,
                hopper_text: Color::LightBlue,
            },

            Theme::Green => ThemeColors {
                accent: Color::LightGreen,
                background: Color::Black,
                banner: Color::LightGreen,

                text: Color::Green,
                active: Color::LightGreen,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::LightGreen,
                hopper_text: Color::LightGreen,
            },
            Theme::Yellow => ThemeColors {
                accent: Color::LightYellow,
                background: Color::Black,
                banner: Color::LightYellow,

                text: Color::Yellow,
                active: Color::LightYellow,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::LightYellow,
                hopper_text: Color::LightYellow,
            },
            Theme::Magenta => ThemeColors {
                accent: Color::LightMagenta,
                background: Color::Black,
                banner: Color::LightMagenta,

                text: Color::Magenta,
                active: Color::LightMagenta,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::LightMagenta,
                hopper_text: Color::LightMagenta,
            },
            Theme::Gray => ThemeColors {
                accent: Color::DarkGray,
                background: Color::Black,
                banner: Color::Gray,

                text: Color::DarkGray,
                active: Color::White,
                warning: Color::LightYellow,
                danger: Color::LightRed,
                success: Color::LightGreen,

                ssh_text: Color::Gray,
                hopper_text: Color::Gray,
            },
        }
    }
}
