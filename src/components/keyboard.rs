use dioxus::prelude::*;
use models::kb_layout::{KbLayout, Key};

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
        keyboard_row( id: String::from("row_numbers"), keys: row_numbers )
        keyboard_row( id: String::from("row_1"), keys: row_1 )
        keyboard_row( id: String::from("row_2"), keys: row_2 )
        keyboard_row( id: String::from("row_3"), keys: row_3 )
        keyboard_row( id: String::from("row_space"), keys: row_space )
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
    cx.render(match cx.props.cmd.as_str() {
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
    })
}
