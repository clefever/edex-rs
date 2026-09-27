use dioxus::{
    desktop::{Config, LogicalSize, WindowBuilder, wry::dpi::Size},
    prelude::*,
};

mod components;
mod models;

const FONT_FIRA_CODE: Asset = asset!("/assets/fonts/fira_code.woff2");
const FONT_FIRA_MONO: Asset = asset!("/assets/fonts/fira_mono.woff2");
const FONT_UNITED_SANS_LIGHT: Asset = asset!("/assets/fonts/united_sans_light.woff2");
const FONT_UNITED_SANS_MEDIUM: Asset = asset!("/assets/fonts/united_sans_medium.woff2");

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
                    .with_inner_size(Size::Logical(LogicalSize::new(1280.0, 720.0))),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    // TODO: settings `nointro`/`nointroOverride` should skip straight to the main UI
    let mut boot_done = use_signal(|| false);
    let kb_layout = use_signal(load_kb_layout);
    let theme = load_theme().to_css_vars();

    rsx! {
        document::Style { class: "theming", "{theme}" }
        document::Style { class: "fonts", "{fonts_css()}" }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: MAIN_SHELL_CSS }
        document::Stylesheet { href: MOD_COLUMN_CSS }
        document::Stylesheet { href: EXTRA_RATIOS_CSS }
        if !boot_done() {
            body {
                class: "solidBackground",
                components::BootScreen { on_done: move || boot_done.set(true) }
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
    serde_json::from_str(theme).expect("bundled themes/tron.json failed to parse")
}

fn load_kb_layout() -> models::KbLayout {
    let layout = include_str!("../assets/kb_layouts/en-US.json");
    models::KbLayout::from_json_str(layout).expect("bundled kb_layouts/en-US.json failed to parse")
}

fn fonts_css() -> String {
    format!(
        "
    @font-face {{
        font-family: 'Fira Code';
        src: url('{}') format('woff2');
    }}

    @font-face {{
        font-family: 'Fira Mono';
        src: url('{}') format('woff2');
    }}

    @font-face {{
        font-family: 'United Sans Light';
        src: url('{}') format('woff2');
    }}

    @font-face {{
        font-family: 'United Sans Medium';
        src: url('{}') format('woff2');
    }}
    ",
        FONT_FIRA_CODE, FONT_FIRA_MONO, FONT_UNITED_SANS_LIGHT, FONT_UNITED_SANS_MEDIUM,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonts_css_declares_all_four_faces() {
        let css = fonts_css();
        for family in [
            "Fira Code",
            "Fira Mono",
            "United Sans Light",
            "United Sans Medium",
        ] {
            assert!(
                css.contains(&format!("font-family: '{family}';")),
                "missing @font-face for {family}"
            );
        }
        assert_eq!(
            css.matches("@font-face").count(),
            4,
            "expected exactly four @font-face blocks"
        );
    }
}
