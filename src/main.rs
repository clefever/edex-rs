use dioxus::prelude::*;
use models::kb_layout::KbLayout;
use models::theme::Theme;

mod components {
    pub mod boot_screen;
    pub mod keyboard;
}

mod models {
    pub mod kb_layout;
    pub mod theme;
}

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
        body { class: "solidBackground",
            components::boot_screen::boot_screen()
            components::keyboard::keyboard(layout: kb_layout)
        }
    ))
}

fn load_theme() -> Theme {
    let theme = include_str!("./assets/themes/tron.json");
    let result: Theme = serde_json::from_str(theme).unwrap();
    result
}

fn load_kb_layout() -> KbLayout {
    let layout = include_str!("./assets/kb_layouts/en-US.json");
    let result: KbLayout = serde_json::from_str(layout).unwrap();
    result
}

fn theme_str(theme: Theme) -> String {
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
