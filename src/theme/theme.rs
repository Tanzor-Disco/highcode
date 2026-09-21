use crate::cli::input::ThemeChoice;

pub struct ThemeColor {
    r: u8,
    g: u8,
    b: u8,
}

impl ThemeColor {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        ThemeColor { r, g, b }
    }
    pub fn to_css(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct Theme {
    pub keyword: ThemeColor,
    pub function: ThemeColor,
    pub string: ThemeColor,
    pub comment: ThemeColor,
    pub background: ThemeColor,
    pub main_text: ThemeColor,
}

const DARK_THEME: Theme = Theme {
    keyword: ThemeColor::new(255, 215, 0),
    function: ThemeColor::new(207, 214, 241),
    string: ThemeColor::new(140, 215, 83),
    comment: ThemeColor::new(209, 154, 102),
    background: ThemeColor::new(13, 17, 23),
    main_text: ThemeColor::new(207, 214, 241),
};

const LIGHT_THEME: Theme = Theme {
    keyword: ThemeColor::new(175, 0, 219),
    function: ThemeColor::new(121, 94, 38),
    string: ThemeColor::new(163, 21, 21),
    comment: ThemeColor::new(0, 128, 0),
    background: ThemeColor::new(255, 255, 255),
    main_text: ThemeColor::new(36, 41, 47),
};

pub fn get_theme(choice: &ThemeChoice) -> Theme {
    match choice {
        ThemeChoice::Dark => DARK_THEME,
        ThemeChoice::Light => LIGHT_THEME,
    }
}
