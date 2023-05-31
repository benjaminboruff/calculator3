use evalexpr::*;
//use log::info;
use sycamore::prelude::*;

use crate::{InputValue, OutputValue};

#[derive(Props)]
pub struct KeyProps {
    key: &'static str,
    value: &'static str,
}

// fn handle_button_click(item: &str) -> f64 {
//     item.parse::<f64>().unwrap()
// }

#[component]
pub fn Key<G: Html>(cx: Scope, props: KeyProps) -> View<G> {
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    let state_input_value: &Signal<InputValue> = use_context(cx);

    let handle_click = |button: &'static str| match button {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "."
        | "*" | "+" | "-" | "/" => {
            //state_output_number.set(OutputValue(handle_button_click(button)));
            let new_input = state_input_value.get().value().to_owned() + button;
            state_input_value.set(InputValue(new_input));
        }
        "clear" => {
            state_output_number.set(OutputValue(0.0));
            state_input_value.set(InputValue("".to_string()));
        }
        "=" => {
            let calculated_val = eval(state_input_value.get().value().as_str());
            if let Ok(new_value) = calculated_val {
                let converted_val = new_value.as_number(); // int vals are converted to floats
                if let Ok(val) = converted_val {
                    state_output_number.set(OutputValue(val));
                }
            }
        }
        _ => {
            state_output_number.set(OutputValue(0.0));
            state_input_value.set(InputValue(button.to_string()));
        }
    };

    view! {cx,
        button(on:click=move|_| handle_click(props.value), id=props.key, class="bg-blue-200 rounded-lg min-w-full") { (props.value) }
    }
}
