use chrono::Local;
use dioxus::prelude::*;
use tokio::time::Duration;

const CLOCK_CSS: Asset = asset!("/assets/css/mod_clock.css");

#[component]
pub fn Clock() -> Element {
    let now = Local::now();

    let mut time = use_signal(|| now.format("%T").to_string());

    spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            let now = Local::now();
            *time.write() = now.format("%T").to_string();
        }
    });

    rsx! {
        document::Stylesheet { href: CLOCK_CSS }
        div { id: "mod_clock", class: "", // TODO: Make configurable
            style: "animation-play-state: running;", // TODO: Set by timed startup
            h1 { id: "mod_clock_text",
                TimeDigits { time: time() }
            }
        }
    }
}

#[component]
fn TimeDigits(time: String) -> Element {
    rsx! {
        {time.chars().map(|char| {
            match char {
                ':' => rsx! { em { "{char}" } },
                _ => rsx! { span { "{char}" } },
            }
        })}
    }
}
