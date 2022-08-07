use serde::Deserialize;

#[derive(Deserialize)]
pub struct Theme {
    pub colors: Colors,
    #[serde(rename = "cssvars")]
    pub css_vars: CssVars,
    pub terminal: Terminal,
    pub globe: Globe,
}

#[derive(Deserialize)]
pub struct Colors {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub black: String,
    pub light_black: String,
    pub grey: String,
    pub red: Option<String>,
    pub yellow: Option<String>,
}

#[derive(Deserialize)]
pub struct CssVars {
    pub font_main: String,
    pub font_main_light: String,
}

#[derive(Deserialize)]
pub struct Terminal {
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "cursorStyle")]
    pub cursor_style: String,
    pub foreground: String,
    pub background: String,
    pub cursor: String,
    #[serde(rename = "cursorAccent")]
    pub cursor_accent: String,
    pub selection: String,
}

#[derive(Deserialize)]
pub struct Globe {
    pub base: String,
    pub marker: String,
    pub pin: String,
    pub satellite: String,
}
