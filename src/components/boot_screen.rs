use dioxus::prelude::*;
use std::fs;
use tokio::time::Duration;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const BOOT_SCREEN_CSS: Asset = asset!("/assets/css/boot_screen.css");

#[derive(Clone, Copy, PartialEq)]
enum State {
    Boot,
    TitleTransition,
    TitleAppear,
    TitleFill,
    TitleBorder,
    TitleGlitch,
    TitleSettled,
}

const BOOT_TITLE_CSS: &str = "
section#boot_screen h1.title_fill {
    background-color: rgb(var(--color_r), var(--color_g), var(--color_b));
    border-bottom: 5px solid rgb(var(--color_r), var(--color_g), var(--color_b));
}
section#boot_screen h1.title_border {
    border: 5px solid rgb(var(--color_r), var(--color_g), var(--color_b));
}
";

#[component]
fn BootTitleStyle() -> Element {
    rsx! {
        document::Style { class: "boot_title", {BOOT_TITLE_CSS} }
    }
}

fn title_class(state: State) -> &'static str {
    match state {
        State::TitleFill => "title_fill",
        State::TitleBorder | State::TitleSettled => "title_border",
        State::TitleGlitch => "glitch",
        _ => "",
    }
}

#[component]
pub fn BootScreen(on_done: EventHandler) -> Element {
    let mut state = use_signal(|| State::Boot);
    let mut curr_line: Signal<u32> = use_signal(|| 0);
    let mut lines = use_signal(Vec::new);
    let mut class = use_signal(String::new);

    // Time format example: "Fri Jul 15 2022 14:35:43 GMT-0400 (Eastern Daylight Time)"

    use_future(move || async move {
        let all_lines = load_boot_log();

        while (curr_line() as usize) < all_lines.len() {
            let dur = timeout_from_line(curr_line(), all_lines.len());
            tokio::time::sleep(Duration::from_millis(dur)).await;
            let line = all_lines[curr_line() as usize].to_owned();
            lines.with_mut(|lines| lines.push(line));
            if curr_line() == 1 {
                lines.with_mut(|lines| {
                    lines.push(format!(
                        "eDEX-rs Kernel version {} boot at {}; root:xnu-1699.22.73~1/RELEASE_X86_64",
                        VERSION.unwrap_or("unknown"),
                        "FIXME"
                    ))
                });
            } else if curr_line() == 82 && is_arch_user() {
                lines.with_mut(|lines| lines.push(String::from("btw i use arch")));
            }
            curr_line.with_mut(|line| *line += 1);
        }

        // TODO: upstream also toggles the body solidBackground class here.
        tokio::time::sleep(Duration::from_millis(300)).await;
        state.set(State::TitleTransition);
        tokio::time::sleep(Duration::from_millis(400)).await;
        class.set("center".to_string());
        state.set(State::TitleAppear);
        tokio::time::sleep(Duration::from_millis(300)).await;
        state.set(State::TitleFill);
        tokio::time::sleep(Duration::from_millis(300)).await;
        state.set(State::TitleBorder);
        tokio::time::sleep(Duration::from_millis(100)).await;
        state.set(State::TitleGlitch);
        tokio::time::sleep(Duration::from_millis(500)).await;
        state.set(State::TitleSettled);
        tokio::time::sleep(Duration::from_millis(1000)).await;
        on_done.call(());
    });

    let boot_lines = lines.cloned();

    rsx! {
        document::Stylesheet { href: BOOT_SCREEN_CSS }
        BootTitleStyle {}
        section { class: "{class}", id: "boot_screen",
            match *state.read() {
                State::Boot => rsx! {
                    {boot_lines.iter().map(|line: &String| {
                        rsx! {
                            "{line}"
                            br {}
                        }
                    })}
                },
                State::TitleTransition => rsx! { "" },
                title => rsx! {
                    h1 { class: "{title_class(title)}", "eDEX-rs" }
                }
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
        43..=81 | 83 => 25,
        x if x as usize >= total_lines.saturating_sub(2) && (x as usize) < total_lines => 300,
        _ => (f32::powi(1.0 - (line_num as f32 / 1000.0), 3) * 25.0).round() as u64,
    }
}

fn load_boot_log() -> Vec<String> {
    let layout = include_str!("../../assets/misc/boot_log.txt");
    layout.lines().map(str::to_string).collect()
}

fn is_arch_user() -> bool {
    match fs::read_to_string("/etc/os-release") {
        Ok(str) => str.contains("arch"),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_lines_get_their_pause_times() {
        let total = 100;
        assert_eq!(timeout_from_line(1, total), 500); // line 2
        assert_eq!(timeout_from_line(3, total), 500); // line 4
        assert_eq!(timeout_from_line(10, total), 30); // lines 5..=24
        assert_eq!(timeout_from_line(24, total), 400); // line 25
        assert_eq!(timeout_from_line(41, total), 300); // line 42 pauses, not 25
        assert_eq!(timeout_from_line(42, total), 25); // line 43
        assert_eq!(timeout_from_line(82, total), 25); // line 83
    }

    #[test]
    fn last_two_lines_pause_before_finishing() {
        let total = 100;
        assert_eq!(timeout_from_line(97, total), 300); // line 98 == total - 2
        assert_eq!(timeout_from_line(98, total), 300); // line 99 == total - 1
    }

    #[test]
    fn short_logs_do_not_underflow() {
        // total < 3 used to evaluate `total_lines - 2` and panic in debug builds
        assert_eq!(timeout_from_line(0, 2), 300); // line 1 is within the tail window
        assert_eq!(timeout_from_line(0, 1), 25); // guard excluded, falls to default curve
        assert_eq!(timeout_from_line(0, 0), 25); // guard never matches
    }

    #[test]
    fn default_curve_decays_slowly() {
        // Line 30 falls through to the powi curve: (1 - 30/1000)^3 * 25
        let expected = (f32::powi(1.0 - 0.03, 3) * 25.0).round() as u64;
        assert_eq!(timeout_from_line(29, 1000), expected);
    }

    #[test]
    fn bundled_boot_log_loads_and_completes() {
        let lines = load_boot_log();
        assert!(lines.len() > 50, "boot log unexpectedly short");
        assert!(
            lines.iter().any(|l| l == "Boot Complete"),
            "boot log never reaches Boot Complete"
        );
    }
}
