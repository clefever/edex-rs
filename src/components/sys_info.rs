use chrono::Local;
use dioxus::prelude::*;

pub fn sys_info(cx: Scope) -> Element {
    let time = Local::now();

    let year = use_state(&cx, || time.format("%Y"));
    let day = use_state(&cx, || time.format("%b %e"));
    // TODO: Month should be capital

    cx.render(rsx!(
        style{ include_str!("../assets/css/mod_sysinfo.css") }
        div { id: "mod_sysinfo", style: "animation-play-state: running;", // TODO: Set by timed startup
            div {
                h1 { "{year}" }
                h2 { "{day}" }
            }
            div {
                h1 { "UPTIME" }
                h2 { "0:0:0" }
            }
            div {
                h1 { "TYPE" }
                h2 { "os" } // TODO: Determine os
            }
            div {
                h1 { "POWER" }
                h2 { "00%" }
            }
        }
    ))
}
