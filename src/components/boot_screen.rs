use dioxus::{core::to_owned, prelude::*};
use tokio::time::Duration;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn boot_screen(cx: Scope) -> Element {
    let curr_line: &UseState<u32> = use_state(&cx, || 0);
    let lines = use_ref(&cx, || vec![]);

    let all_lines = load_boot_log();
    let total_lines = all_lines.len();

    // Time format example: "Fri Jul 15 2022 14:35:43 GMT-0400 (Eastern Daylight Time)"

    cx.spawn({
        to_owned![curr_line, lines];
        async move {
            let dur = timeout_from_line(*curr_line, total_lines);
            tokio::time::sleep(Duration::from_millis(dur)).await;
            if (*curr_line as usize) < total_lines {
                let line = all_lines[*curr_line as usize].to_owned();
                lines.write().push(line);
                if *curr_line == 1 {
                    lines.write().push(format!("eDEX-UI Kernel version {} boot at {}; root:xnu-1699.22.73~1/RELEASE_X86_64", VERSION.unwrap_or("unknown"), "FIXME"));
                } else if *curr_line == 82 && is_arch_user() {
                    lines.write().push(String::from("btw i use arch"));
                }
                curr_line += 1;
            }
        }
    });

    cx.render(rsx!(
        style { [include_str!("../assets/css/boot_screen.css")]}
        section { id: "boot_screen",
        lines.read().iter().map(|line| {
            rsx!(
                "{line}"
                br {}
            )
        })
    }))
}

fn timeout_from_line(line_num: u32, total_lines: usize) -> u64 {
    let line_num = line_num + 1;
    match line_num {
        2 | 4 => 500,
        5..=24 => 30,
        25 => 400,
        42 => 300,
        42..=81 | 83 => 25,
        x if x as usize >= total_lines - 2 && (x as usize) < total_lines => 300,
        _ => (f32::powi(1.0 - (line_num as f32 / 1000.0), 3) * 25.0).round() as u64,
    }
}

fn load_boot_log() -> Vec<String> {
    let layout = include_str!("../assets/misc/boot_log.txt");
    layout.split('\n').map(str::to_string).collect()
}

fn is_arch_user() -> bool {
    false // TODO: Add arch detection logic
}
