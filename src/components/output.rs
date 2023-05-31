use sycamore::prelude::*;

use crate::InputValue;
use crate::OutputValue;

#[component]
pub fn Output<G: Html>(cx: Scope) -> View<G> {
    let state_input_text: &Signal<InputValue> = use_context(cx);
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    view! {cx,
        div {
            p(class="ml-2") { "Input: " (state_input_text.get().value()) }
            p(class="ml-2") { "Result: " (state_output_number.get().value()) }
        }
    }
}
