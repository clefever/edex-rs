use dioxus::prelude::*;

mod components;
mod models;

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_title("eDEX-rs")
                .with_resizable(false)
                .with_inner_size(LogicalSize::new(1280.0, 720.0))
        })
    });
}

fn app(cx: Scope) -> Element {
    let init_ui = use_state(&cx, || true);
    let kb_layout = use_state(&cx, || load_kb_layout());
    let theme = load_theme();
    let theme = theme_str(theme);

    cx.render(rsx!(
        head {
            style { [include_str!("./assets/css/fonts.css")] }
            style { class: "theming",
                dangerous_inner_html: "{theme}" // TODO: Better way to do this?
            }
        }
        style { [include_str!("./assets/css/main.css")] }
        style { [include_str!("./assets/css/main_shell.css")] }
        style { [include_str!("./assets/css/mod_column.css")] }
        style { [include_str!("./assets/css/extra_ratios.css")] }
        if !init_ui {rsx!(
            body { class: "solidBackground",
                components::boot_screen()
            }
        )} else {rsx!(
            body { class: "solidBackground",
                section { class: "mod_column activated", id: "mod_column_left",
                    h3 { class: "title",
                        p { "PANEL" }
                        p { "SYSTEM" }
                    }
                    components::clock()
                    components::sys_info()
                    components::hardware_inspector()
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
                components::keyboard(layout: kb_layout)
            }
        )}
    ))
}

fn load_theme() -> models::Theme {
    let theme = include_str!("./assets/themes/tron.json");
    let result: models::Theme = serde_json::from_str(theme).unwrap();
    result
}

fn load_kb_layout() -> models::KbLayout {
    let layout = include_str!("./assets/kb_layouts/en-US.json");
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
