use chrono::Local;
use dioxus::prelude::*;

const SYS_INFO_CSS: &str = include_str!("../../assets/css/mod_sysinfo.css");

#[component]
pub fn SysInfo() -> Element {
    let time = Local::now();

    let year = use_signal(|| time.format("%Y"));
    let day = use_signal(|| time.format("%b %e"));
    // TODO: Month should be capital

    rsx! {
        style{ "{SYS_INFO_CSS}" }
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
    }
}
