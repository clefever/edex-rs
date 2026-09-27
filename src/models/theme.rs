use serde::Deserialize;

#[derive(Deserialize)]
pub struct Theme {
    pub colors: Colors,
    #[serde(rename = "cssvars")]
    pub css_vars: CssVars,
    pub terminal: Terminal,
    #[allow(dead_code)] // Read once the globe component lands
    pub globe: Globe,
}

impl Theme {
    /// Renders the theme as the `:root` CSS-variable block injected at startup.
    pub fn to_css_vars(&self) -> String {
        format!(
            "
    :root {{
        --font_main: \"{}\";
        --font_main_light: \"{}\";
        --font_mono: \"{}\";
        --color_r: {};
        --color_g: {};
        --color_b: {};
        --color_black: {};
        --color_light_black: {};
        --color_grey: {};

        /* Used for error and warning modals */
        --color_red: {};
        --color_yellow: {};
    }}
    body {{
        font-family: var(--font_main), sans-serif;
        cursor: none !important;
    }}
    * {{
       cursor: none !important;
    }}
    ",
            self.css_vars.font_main,
            self.css_vars.font_main_light,
            self.terminal.font_family,
            self.colors.r,
            self.colors.g,
            self.colors.b,
            self.colors.black,
            self.colors.light_black,
            self.colors.grey,
            self.colors.red.as_deref().unwrap_or("red"),
            self.colors.yellow.as_deref().unwrap_or("yellow"),
        )
    }
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
// Terminal colors are applied once the terminal component lands; the fields
// must stay so upstream theme JSONs keep deserializing.
#[allow(dead_code)]
pub struct Terminal {
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "cursorStyle")]
    pub cursor_style: String,
    pub foreground: String,
    pub background: String,
    pub cursor: String,
    #[serde(rename = "cursorAccent")]
    pub cursor_accent: Option<String>,
    pub selection: String,
}

#[derive(Deserialize)]
// Kept whole for upstream theme deserialization; read by the globe component.
#[allow(dead_code)]
pub struct Globe {
    pub base: String,
    pub marker: String,
    pub pin: String,
    pub satellite: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_bundled_themes_deserialize_and_render() {
        let themes_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/themes");
        let mut count = 0;
        for entry in std::fs::read_dir(&themes_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            let raw = std::fs::read_to_string(&path).unwrap();
            let theme: Theme = serde_json::from_str(&raw)
                .unwrap_or_else(|e| panic!("{} failed to deserialize: {e}", path.display()));
            let css = theme.to_css_vars();

            assert!(
                css.contains(":root {"),
                "{}: no :root block",
                path.display()
            );
            for var in [
                "--font_main:",
                "--font_main_light:",
                "--font_mono:",
                "--color_r:",
                "--color_g:",
                "--color_b:",
                "--color_black:",
                "--color_light_black:",
                "--color_grey:",
                "--color_red:",
                "--color_yellow:",
            ] {
                assert!(css.contains(var), "{}: missing {var}", path.display());
            }
            count += 1;
        }
        assert!(
            count > 0,
            "no theme JSONs found in {}",
            themes_dir.display()
        );
    }

    #[test]
    fn missing_red_yellow_fall_back_to_defaults() {
        let raw = r##"{
            "colors": {
                "r": 0, "g": 255, "b": 190,
                "black": "#073642", "light_black": "#002B36", "grey": "#2AA198",
                "red": null, "yellow": null
            },
            "cssvars": { "font_main": "Fira Mono", "font_main_light": "Fira Sans Light" },
            "terminal": {
                "fontFamily": "Fira Mono", "cursorStyle": "block",
                "foreground": "#93A1A1", "background": "#002B36",
                "cursor": "#93A1A1", "cursorAccent": "#002B36", "selection": "#002B36"
            },
            "globe": {
                "base": "#002B36", "marker": "#268BD2",
                "pin": "#93A1A1", "satellite": "#33DFCC"
            }
        }"##;
        let theme: Theme = serde_json::from_str(raw).unwrap();
        let css = theme.to_css_vars();
        assert!(css.contains("--color_red: red;"));
        assert!(css.contains("--color_yellow: yellow;"));
    }
}
