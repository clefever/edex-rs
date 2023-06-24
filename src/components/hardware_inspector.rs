use dioxus::prelude::*;

pub fn hardware_inspector(cx: Scope) -> Element {
    cx.render(rsx!(
        style { include_str!("../assets/css/mod_hardwareInspector.css") }
        div { id: "mod_hardwareInspector", style: "animation-play-state: running;", // TODO: Set by timed startup
            div { id: "mod_hardwareInspector_inner",
                div {
                    h1 { "MANUFACTURER" }
                    h2 { id: "mod_hardwareInspector_manufacturer",
                        "NONE"
                    }
                }
                div {
                    h1 { "MODEL" }
                    h2 { id: "mod_hardwareInspector_model",
                        "NONE"
                    }
                }
                div {
                    h1 { "CHASSIS" }
                    h2 { id: "mod_hardwareInspector_chassis",
                        "NONE"
                    }
                }
            }
        }
    ))
}
