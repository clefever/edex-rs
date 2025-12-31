use dioxus::prelude::*;

const HARDWARE_INSPECTOR_CSS: Asset = asset!("/assets/css/mod_hardwareInspector.css");

#[component]
pub fn HardwareInspector() -> Element {
    rsx! {
        document::Stylesheet { href: HARDWARE_INSPECTOR_CSS }
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
    }
}
