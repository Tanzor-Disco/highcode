enum ThemeKind {
    Default,
}

struct ThemeColor {
    r: u8,
    g: u8,
    b: u8,
}

struct Theme {
    keyword: ThemeColor,
    function: ThemeColor,
    string: ThemeColor,
    comment: ThemeColor,
    number: ThemeColor,
    type_: ThemeColor,
    variable: ThemeColor,
    background: ThemeColor,
    foreground: ThemeColor,
}
