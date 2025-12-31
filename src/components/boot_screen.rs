use dioxus::prelude::*;
use std::fs;
use tokio::time::Duration;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const BOOT_SCREEN_CSS: Asset = asset!("/assets/css/boot_screen.css");

#[derive(Clone, Copy, PartialEq)]
enum State {
    Boot,
    TitleTransition,
    TitleScreen,
}

#[component]
pub fn BootScreen() -> Element {
    let mut state = use_signal(|| State::Boot);
    let curr_line: Signal<u32> = use_signal(|| 0);
    let lines = use_signal(|| vec![]);
    let class = use_signal(|| String::new());

    let all_lines = load_boot_log();

    // Time format example: "Fri Jul 15 2022 14:35:43 GMT-0400 (Eastern Daylight Time)"
    
    spawn(async move {
        match state() {
            State::Boot => print_line(curr_line, state, lines, all_lines.clone()).await,
            State::TitleTransition => {
                tokio::time::sleep(Duration::from_millis(400)).await;
                state.set(State::TitleScreen);
            }
            State::TitleScreen => display_title(class).await,
        }
    });

    rsx! {
        document::Stylesheet { href: BOOT_SCREEN_CSS }
        section { class: "{class}", id: "boot_screen",
            match *state.read() {
                State::Boot => {rsx! {
                    {lines.read().iter().map(|line: &String| {
                        rsx! {
                            "{line}"
                            br {}
                        }
                    })}
                }},
                State::TitleTransition => rsx! { "" },
                State::TitleScreen => rsx! { h1 { "eDEX-rs" } },
            }
        }
    }
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

async fn print_line(
    mut curr_line: Signal<u32>,
    mut state: Signal<State>,
    mut lines: Signal<Vec<String>>,
    all_lines: Vec<String>,
) {
    if (curr_line() as usize) < all_lines.len() {
        let dur = timeout_from_line(curr_line(), all_lines.len());
        tokio::time::sleep(Duration::from_millis(dur)).await;
        if (curr_line() as usize) < all_lines.len() {
            let line = all_lines[curr_line() as usize].to_owned();
            lines.with_mut(|lines| lines.push(line));
            if curr_line() == 1 {
                lines.with_mut(|lines| {
                    lines.push(format!(
                    "eDEX-UI Kernel version {} boot at {}; root:xnu-1699.22.73~1/RELEASE_X86_64",
                    VERSION.unwrap_or("unknown"),
                    "FIXME"
                ))
                });
            } else if curr_line() == 82 && is_arch_user() {
                lines.with_mut(|lines| lines.push(String::from("btw i use arch")));
            }
            curr_line.with_mut(|line| *line += 1);
        }
    } else {
        tokio::time::sleep(Duration::from_millis(300)).await;
        state.set(State::TitleTransition);
    }
}

async fn display_title(mut class: Signal<String>) {
    if class() != "center" {
        class.set("center".to_string());
        // TODO: Some stuff that requires theme vars
        tokio::time::sleep(Duration::from_millis(700)).await;
    }
}

fn load_boot_log() -> Vec<String> {
    let layout = include_str!("../../assets/misc/boot_log.txt");
    layout.split('\n').map(str::to_string).collect()
}

fn is_arch_user() -> bool {
    match fs::read_to_string("/etc/os-release") {
        Ok(str) => str.contains("arch"),
        Err(_) => false,
    }
}
