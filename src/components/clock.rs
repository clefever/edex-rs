use chrono::Local;
use dioxus::{core::to_owned, prelude::*};
use tokio::time::Duration;

pub fn clock(cx: Scope) -> Element {
    let now = Local::now();

    let time = use_state(&cx, || now.format("%T"));

    cx.spawn({
        to_owned![time];
        async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            let now = Local::now();
            time.set(now.format("%T"));
        }
    });

    cx.render(rsx!(
        style{ [include_str!("../assets/css/mod_clock.css")] }
        div { id: "mod_clock", class: "", // TODO: Make configurable
            style: "animation-play-state: running;", // TODO: Set by timed startup
            h1 { id:"mod_clock_text",
                time_digits(time: time.to_string())
            }
        }
    ))
}

#[derive(PartialEq, Props)]
struct TimeDigitsProps {
    time: String,
}

fn time_digits(cx: Scope<TimeDigitsProps>) -> Element {
    cx.render(rsx!(cx.props.time.chars().map(|char| {
        rsx!(match char {
            ':' => rsx!(em { "{char}" }),
            _ => rsx!(span { "{char}" }),
        })
    })))
}
