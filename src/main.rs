use dioxus::{desktop::{Config, LogicalSize, WindowBuilder, wry::dpi::Size}, prelude::*};

mod components;
mod models;

const FONTS_CSS: Asset = asset!("/assets/css/fonts.css");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const MAIN_SHELL_CSS: Asset = asset!("/assets/css/main_shell.css");
const MOD_COLUMN_CSS: Asset = asset!("/assets/css/mod_column.css");
const EXTRA_RATIOS_CSS: Asset = asset!("/assets/css/extra_ratios.css");

fn main() {
    dioxus::LaunchBuilder::new()
    .with_cfg(
        Config::default().with_menu(None).with_window(
            WindowBuilder::new()
                .with_title("eDEX-rs")
                .with_resizable(false)
                .with_inner_size(Size::Logical(LogicalSize::new(1280.0, 720.0)))
            )
        )
    .launch(App);
}

#[component]
fn App() -> Element {
    let init_ui = use_signal(|| true);
    let kb_layout = use_signal(|| load_kb_layout());
    let theme = load_theme();
    let theme = theme_str(theme);
    
    rsx! {
        style { class: "theming",
            dangerous_inner_html: "{theme}" // TODO: Better way to do this?
        }
        document::Stylesheet { href: FONTS_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: MAIN_SHELL_CSS }
        document::Stylesheet { href: MOD_COLUMN_CSS }
        document::Stylesheet { href: EXTRA_RATIOS_CSS }
        if !init_ui() {
            body {
                class: "solidBackground",
                components::BootScreen {}
            }
        } else {
            body { class: "solidBackground",
                section { class: "mod_column activated", id: "mod_column_left",
                    h3 { class: "title",
                        p { "PANEL" }
                        p { "SYSTEM" }
                    }
                    components::Clock {}
                    components::SysInfo {}
                    components::HardwareInspector {}
                }
                section { id: "main_shell", style: "margin-bottom:30vh;", "augmented-ui": "bl-clip tr-clip exe",
                    h3 { class: "title", style: "",
                        p { "TERMINAL" }
                        p { "MAIN SHELL" }
                    }
                    h1 { id: "main_shell_greeting" }
                }
                section { class: "mod_column activated", id: "mod_column_right",
                    h3 { class: "title",
                        p { "PANEL" }
                        p { "NETWORK" }
                    }
                }
                components::Keyboard { layout: kb_layout }
            }
        }
    }
}

fn load_theme() -> models::Theme {
    let theme = include_str!("../assets/themes/tron.json");
    let result: models::Theme = serde_json::from_str(theme).unwrap();
    result
}

fn load_kb_layout() -> models::KbLayout {
    let layout = include_str!("../assets/kb_layouts/en-US.json");
    let result: models::KbLayout = serde_json::from_str(layout).unwrap();
    result
}

fn theme_str(theme: models::Theme) -> String {
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
        theme.css_vars.font_main,
        theme.css_vars.font_main_light,
        theme.terminal.font_family,
        theme.colors.r,
        theme.colors.g,
        theme.colors.b,
        theme.colors.black,
        theme.colors.light_black,
        theme.colors.grey,
        theme.colors.red.unwrap_or(String::from("red")),
        theme.colors.yellow.unwrap_or(String::from("yellow")),
    )
}
