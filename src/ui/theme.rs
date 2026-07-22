use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub panel_bg: Color,
    pub selection_bg: Color,
    pub border: Color,
    pub border_focused: Color,
    pub accent: Color,
    pub secondary_accent: Color,
    pub success: Color,
    pub warning: Color,
    pub text: Color,
    pub muted: Color,
    pub highlight: Color,
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "catppuccin_mocha",
            bg: Color::Reset,
            panel_bg: Color::Rgb(24, 24, 37),     // Crust / Base
            selection_bg: Color::Rgb(49, 50, 68), // Surface0
            border: Color::Rgb(69, 71, 90),       // Surface2
            border_focused: Color::Rgb(203, 166, 247), // Mauve
            accent: Color::Rgb(203, 166, 247),    // Mauve
            secondary_accent: Color::Rgb(137, 180, 250), // Sapphire Blue
            success: Color::Rgb(166, 227, 161),   // Green
            warning: Color::Rgb(249, 226, 175),   // Yellow
            text: Color::Rgb(205, 214, 244),      // Text
            muted: Color::Rgb(115, 121, 148),     // Subtext0
            highlight: Color::Rgb(250, 179, 135), // Peach
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "tokyo_night",
            bg: Color::Reset,
            panel_bg: Color::Rgb(22, 22, 30),
            selection_bg: Color::Rgb(41, 46, 66),
            border: Color::Rgb(56, 60, 82),
            border_focused: Color::Rgb(122, 162, 247), // Storm Blue
            accent: Color::Rgb(122, 162, 247),         // Storm Blue
            secondary_accent: Color::Rgb(187, 154, 247), // Purple
            success: Color::Rgb(158, 206, 106),        // Green
            warning: Color::Rgb(224, 175, 104),        // Orange
            text: Color::Rgb(192, 202, 245),           // Text
            muted: Color::Rgb(86, 95, 137),            // Muted
            highlight: Color::Rgb(255, 158, 100),      // Peach Accent
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "dracula",
            bg: Color::Reset,
            panel_bg: Color::Rgb(40, 42, 54),
            selection_bg: Color::Rgb(68, 71, 90),
            border: Color::Rgb(98, 114, 164),
            border_focused: Color::Rgb(255, 121, 198),
            accent: Color::Rgb(255, 121, 198),
            secondary_accent: Color::Rgb(189, 147, 249),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(255, 184, 108),
            text: Color::Rgb(248, 248, 242),
            muted: Color::Rgb(139, 150, 185),
            highlight: Color::Rgb(139, 233, 253),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "nord",
            bg: Color::Reset,
            panel_bg: Color::Rgb(46, 52, 64),
            selection_bg: Color::Rgb(59, 66, 82),
            border: Color::Rgb(76, 86, 106),
            border_focused: Color::Rgb(136, 192, 208),
            accent: Color::Rgb(136, 192, 208),
            secondary_accent: Color::Rgb(129, 161, 193),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            text: Color::Rgb(216, 222, 233),
            muted: Color::Rgb(108, 120, 144),
            highlight: Color::Rgb(208, 135, 112),
        }
    }

    pub fn obsidian() -> Self {
        Self {
            name: "obsidian",
            bg: Color::Reset,
            panel_bg: Color::Rgb(24, 24, 32),
            selection_bg: Color::Rgb(35, 40, 50),
            border: Color::Rgb(60, 60, 75),
            border_focused: Color::Rgb(0, 200, 220),
            accent: Color::Rgb(0, 210, 230),
            secondary_accent: Color::Rgb(120, 190, 240),
            success: Color::Rgb(80, 210, 130),
            warning: Color::Rgb(245, 185, 65),
            text: Color::Rgb(215, 215, 225),
            muted: Color::Rgb(105, 110, 130),
            highlight: Color::Rgb(255, 190, 90),
        }
    }

    pub fn solarized() -> Self {
        Self {
            name: "solarized",
            bg: Color::Reset,
            panel_bg: Color::Rgb(7, 54, 66),
            selection_bg: Color::Rgb(18, 70, 84),
            border: Color::Rgb(88, 110, 117),
            border_focused: Color::Rgb(38, 139, 210),
            accent: Color::Rgb(38, 139, 210),
            secondary_accent: Color::Rgb(42, 161, 152),
            success: Color::Rgb(133, 153, 0),
            warning: Color::Rgb(181, 137, 0),
            text: Color::Rgb(147, 161, 161),
            muted: Color::Rgb(101, 123, 131),
            highlight: Color::Rgb(203, 75, 22),
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "gruvbox",
            bg: Color::Reset,
            panel_bg: Color::Rgb(40, 40, 40),
            selection_bg: Color::Rgb(80, 73, 69),
            border: Color::Rgb(124, 111, 100),
            border_focused: Color::Rgb(250, 189, 47),
            accent: Color::Rgb(250, 189, 47),
            secondary_accent: Color::Rgb(104, 157, 106),
            success: Color::Rgb(152, 151, 26),
            warning: Color::Rgb(214, 93, 14),
            text: Color::Rgb(235, 219, 178),
            muted: Color::Rgb(146, 131, 116),
            highlight: Color::Rgb(177, 98, 134),
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            name: "rose_pine",
            bg: Color::Reset,
            panel_bg: Color::Rgb(25, 23, 36),
            selection_bg: Color::Rgb(38, 35, 58),
            border: Color::Rgb(82, 78, 118),
            border_focused: Color::Rgb(235, 111, 146),
            accent: Color::Rgb(235, 111, 146),
            secondary_accent: Color::Rgb(49, 116, 143),
            success: Color::Rgb(156, 207, 136),
            warning: Color::Rgb(246, 193, 119),
            text: Color::Rgb(224, 222, 244),
            muted: Color::Rgb(110, 106, 134),
            highlight: Color::Rgb(196, 167, 231),
        }
    }

    pub fn everforest() -> Self {
        Self {
            name: "everforest",
            bg: Color::Reset,
            panel_bg: Color::Rgb(45, 53, 59),
            selection_bg: Color::Rgb(63, 74, 83),
            border: Color::Rgb(83, 94, 101),
            border_focused: Color::Rgb(167, 192, 128),
            accent: Color::Rgb(167, 192, 128),
            secondary_accent: Color::Rgb(127, 187, 179),
            success: Color::Rgb(131, 192, 146),
            warning: Color::Rgb(230, 126, 128),
            text: Color::Rgb(211, 198, 170),
            muted: Color::Rgb(108, 125, 131),
            highlight: Color::Rgb(214, 153, 182),
        }
    }

    pub fn kanagawa() -> Self {
        Self {
            name: "kanagawa",
            bg: Color::Reset,
            panel_bg: Color::Rgb(31, 31, 40),
            selection_bg: Color::Rgb(54, 53, 67),
            border: Color::Rgb(79, 78, 93),
            border_focused: Color::Rgb(126, 156, 216),
            accent: Color::Rgb(126, 156, 216),
            secondary_accent: Color::Rgb(127, 180, 202),
            success: Color::Rgb(118, 148, 106),
            warning: Color::Rgb(220, 138, 43),
            text: Color::Rgb(220, 215, 186),
            muted: Color::Rgb(112, 116, 135),
            highlight: Color::Rgb(149, 127, 184),
        }
    }

    pub fn monokai_pro() -> Self {
        Self {
            name: "monokai_pro",
            bg: Color::Reset,
            panel_bg: Color::Rgb(45, 42, 46),
            selection_bg: Color::Rgb(67, 63, 68),
            border: Color::Rgb(90, 86, 91),
            border_focused: Color::Rgb(255, 97, 136),
            accent: Color::Rgb(255, 97, 136),
            secondary_accent: Color::Rgb(171, 157, 242),
            success: Color::Rgb(169, 220, 118),
            warning: Color::Rgb(255, 216, 102),
            text: Color::Rgb(252, 252, 250),
            muted: Color::Rgb(126, 122, 128),
            highlight: Color::Rgb(120, 220, 232),
        }
    }

    pub fn one_dark() -> Self {
        Self {
            name: "one_dark",
            bg: Color::Reset,
            panel_bg: Color::Rgb(40, 44, 52),
            selection_bg: Color::Rgb(58, 62, 71),
            border: Color::Rgb(73, 78, 88),
            border_focused: Color::Rgb(224, 108, 117),
            accent: Color::Rgb(97, 175, 239),
            secondary_accent: Color::Rgb(198, 120, 221),
            success: Color::Rgb(152, 195, 121),
            warning: Color::Rgb(229, 192, 123),
            text: Color::Rgb(171, 178, 191),
            muted: Color::Rgb(105, 112, 125),
            highlight: Color::Rgb(86, 182, 194),
        }
    }

    pub fn all() -> Vec<Theme> {
        vec![
            Self::catppuccin_mocha(),
            Self::tokyo_night(),
            Self::dracula(),
            Self::nord(),
            Self::obsidian(),
            Self::solarized(),
            Self::gruvbox(),
            Self::rose_pine(),
            Self::everforest(),
            Self::kanagawa(),
            Self::monokai_pro(),
            Self::one_dark(),
        ]
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "catppuccin_mocha" => Self::catppuccin_mocha(),
            "dracula" => Self::dracula(),
            "everforest" => Self::everforest(),
            "gruvbox" => Self::gruvbox(),
            "kanagawa" => Self::kanagawa(),
            "monokai_pro" => Self::monokai_pro(),
            "nord" => Self::nord(),
            "obsidian" => Self::obsidian(),
            "one_dark" => Self::one_dark(),
            "rose_pine" => Self::rose_pine(),
            "solarized" => Self::solarized(),
            "tokyo_night" => Self::tokyo_night(),
            _ => Self::catppuccin_mocha(),
        }
    }
}
