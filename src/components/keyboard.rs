use dioxus::prelude::*;
use models::{KbLayout, Key};

use crate::models;

#[derive(Props)]
pub struct KeyboardProps<'a> {
    layout: &'a UseState<KbLayout>,
}

pub fn keyboard<'a>(cx: Scope<'a, KeyboardProps<'a>>) -> Element<'a> {
    let row_numbers = cx.props.layout.current().row_numbers.to_vec();
    let row_1 = cx.props.layout.current().row_1.to_vec();
    let row_2 = cx.props.layout.current().row_2.to_vec();
    let row_3 = cx.props.layout.current().row_3.to_vec();
    let row_space = cx.props.layout.current().row_space.to_vec();

    cx.render(rsx!(
        style { [include_str!("../assets/css/keyboard.css")]}
        section { id: "keyboard",
            keyboard_row( id: String::from("row_numbers"), keys: row_numbers )
            keyboard_row( id: String::from("row_1"), keys: row_1 )
            keyboard_row( id: String::from("row_2"), keys: row_2 )
            keyboard_row( id: String::from("row_3"), keys: row_3 )
            keyboard_row( id: String::from("row_space"), keys: row_space )
        }
    ))
}

#[derive(PartialEq, Props)]
struct KeyboardRowProps {
    id: String,
    keys: Vec<Key>,
}

fn keyboard_row(cx: Scope<KeyboardRowProps>) -> Element {
    let keys = cx.props.keys.to_vec();

    cx.render(rsx!(
        div {
            class: "keyboard_row", id: "{cx.props.id}",
            keys.into_iter().map(|key| {
                rsx!(keyboard_key( name: key.name, cmd: key.cmd, altshift_name: key.altshift_name.unwrap_or(String::new()), fn_name: key.fn_name.unwrap_or(String::new()), alt_name: key.alt_name.unwrap_or(String::new()), shift_name: key.shift_name.unwrap_or(String::new()) ))
            })
        }
    ))
}

#[derive(PartialEq, Props)]
struct KeyProps {
    name: String,
    cmd: String,
    altshift_name: String,
    fn_name: String,
    alt_name: String,
    shift_name: String,
}

fn keyboard_key(cx: Scope<KeyProps>) -> Element {
    cx.render(if cx.props.name.starts_with("ESCAPED|-- ICON: ") {
        rsx!(div {
            class: "keyboard_key",
            match &cx.props.name[17..] {
                "ARROW_UP" => rsx!(arrow_up()),
                "ARROW_LEFT" => rsx!(arrow_left()),
                "ARROW_DOWN" => rsx!(arrow_down()),
                "ARROW_RIGHT" => rsx!(arrow_right()),
                _ => rsx!(missing_icon()),
            }
        })
    } else {
        match cx.props.cmd.as_str() {
            " " => rsx!(div {
                class: "keyboard_key",
                id: "keyboard_spacebar"
            }),
            "\r" => rsx!(div { class: "keyboard_key keyboard_enter", h1 { "{cx.props.name}" } }),
            _ => {
                rsx!(div {
                    class: "keyboard_key",
                    h5 { "{cx.props.altshift_name}" }
                    h4 { "{cx.props.fn_name}" }
                    h3 { "{cx.props.alt_name}" }
                    h2 { "{cx.props.shift_name}" }
                    h1 { "{cx.props.name}" }
                })
            }
        }
    })
}

fn arrow_up(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        view_box: "0 0 24.00 24.00",
        path { fill_opacity: "1", d: "m12.00004 7.99999 4.99996 5h-2.99996v4.00001h-4v-4.00001h-3z" }
        path { stroke_linejoin: "round", fill_opacity: "0.65", d: "m4 3h16c1.1046 0 1-0.10457 1 1v16c0 1.1046 0.1046 1-1 1h-16c-1.10457 0-1 0.1046-1-1v-16c0-1.10457-0.10457-1 1-1zm0 1v16h16v-16z"}
    }))
}

fn arrow_left(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        view_box: "0 0 24.00 24.00",
        path { fill_opacity: "1", d: "m7.500015 12.499975 5-4.99996v2.99996h4.00001v4h-4.00001v3z" }
        path { stroke_linejoin: "round", fill_opacity: "0.65", d: "m4 3h16c1.1046 0 1-0.10457 1 1v16c0 1.1046 0.1046 1-1 1h-16c-1.10457 0-1 0.1046-1-1v-16c0-1.10457-0.10457-1 1-1zm0 1v16h16v-16z"}
    }))
}

fn arrow_down(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        view_box: "0 0 24.00 24.00",
        path { fill_opacity: "1", d: "m12 17-4.99996-5h2.99996v-4.00001h4v4.00001h3z" }
        path { stroke_linejoin: "round", fill_opacity: "0.65", d: "m4 3h16c1.1046 0 1-0.10457 1 1v16c0 1.1046 0.1046 1-1 1h-16c-1.10457 0-1 0.1046-1-1v-16c0-1.10457-0.10457-1 1-1zm0 1v16h16v-16z"}
    }))
}

fn arrow_right(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        view_box: "0 0 24.00 24.00",
        path { fill_opacity: "1", d: "m16.500025 12.500015-5 4.99996v-2.99996h-4.00001v-4h4.00001v-3z" }
        path { stroke_linejoin: "round", fill_opacity: "0.65", d: "m4 3h16c1.1046 0 1-0.10457 1 1v16c0 1.1046 0.1046 1-1 1h-16c-1.10457 0-1 0.1046-1-1v-16c0-1.10457-0.10457-1 1-1zm0 1v16h16v-16z"}
    }))
}

fn missing_icon(cx: Scope) -> Element {
    cx.render(rsx!(svg {
        view_box: "0 0 24.00 24.00",
        path { fill: "#ff0000", fill_opacity: "1", d: "M 8.27125,2.9978L 2.9975,8.27125L 2.9975,15.7275L 8.27125,21.0012L 15.7275,21.0012C 17.485,19.2437 21.0013,15.7275 21.0013,15.7275L 21.0013,8.27125L 15.7275,2.9978M 9.10125,5L 14.9025,5L 18.9988,9.10125L 18.9988,14.9025L 14.9025,18.9988L 9.10125,18.9988L 5,14.9025L 5,9.10125M 9.11625,7.705L 7.705,9.11625L 10.5912,12.0025L 7.705,14.8825L 9.11625,16.2937L 12.0025,13.4088L 14.8825,16.2937L 16.2938,14.8825L 13.4087,12.0025L 16.2938,9.11625L 14.8825,7.705L 12.0025,10.5913" }
    }))
}
