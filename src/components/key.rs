use sycamore::prelude::*;

use crate::OutputValue;

#[derive(Props)]
pub struct KeyProps {
    key: &'static str,
    value: &'static str,
}

fn handle_button_click(item: &str) -> f64 {
    match item {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            item.parse::<f64>().unwrap()
        }
        "clear" => 0.0,
        _ => 0.0,
    }
}

#[component]
pub fn Key<G: Html>(cx: Scope, props: KeyProps) -> View<G> {
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    view! {cx,
        button(on:click=|_| state_output_number.set(OutputValue(handle_button_click(props.value))), id=props.key, class="bg-blue-200 rounded-lg min-w-full") { (props.value) }
    }
}
