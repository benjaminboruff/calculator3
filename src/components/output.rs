use crate::{InputValue, OutputValue};
use sycamore::prelude::*;

#[component]
pub fn Output<G: Html>(cx: Scope) -> View<G> {
    let state_input_text: &Signal<InputValue> = use_context(cx);
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    view! {cx,
        div {
            p { "Result: " (state_output_number.get().value()) }
            p { "Input: " (state_input_text.get().value()) }
        }
    }
}
