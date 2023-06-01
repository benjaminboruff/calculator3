use crate::{InputValue, OutputValue};
use evalexpr::*;
use sycamore::prelude::*;

#[derive(Props)]
pub struct KeyProps<'cx, G: Html> {
    key: &'static str,
    value: &'static str,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn Key<'cx, G: Html>(cx: Scope<'cx>, props: KeyProps<'cx, G>) -> View<G> {
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    let state_input_value: &Signal<InputValue> = use_context(cx);

    let handle_click = |button: &'static str| match button {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
            let new_input = state_input_value.get().value().to_owned() + button;
            state_input_value.set(InputValue(new_input));
        }
        "+" | "-" | "/" | "*" => {
            let new_input =
                state_input_value.get().value().to_owned() + " " + button + " ";
            state_input_value.set(InputValue(new_input));
        }
        "clear" => {
            state_output_number.set(OutputValue(0.0));
            state_input_value.set(InputValue("".to_string()));
        }
        "=" => {
            if state_input_value.get().is_valid() {
                let calculated_val =
                    eval_number(state_input_value.get().value().as_str());
                if let Ok(new_value) = calculated_val {
                    let converted_val = new_value;
                    state_output_number.set(OutputValue(converted_val));
                }
            } else {
                state_input_value
                    .set(InputValue("Invalid expression".to_string()));
            }
        }
        "undo" => {
            let mut new_input = state_input_value.get().value().clone();
            if let Some(_item) = new_input.pop() {
                state_input_value.set(InputValue(new_input));
            } else {
                state_input_value.set(InputValue("".to_string()));
            }
        }
        _ => {
            state_output_number.set(OutputValue(0.0));
            state_input_value.set(InputValue("".to_string()));
        }
    };

    view! {cx,
        div( ..props.attributes ) {
            button(class="w-full", on:click=move|_| handle_click(props.value), id=props.key) { (props.value) }
        }
    }
}
