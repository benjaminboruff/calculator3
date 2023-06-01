use crate::{InputValue, OutputValue};
use sycamore::prelude::*;

#[component]
pub fn Output<G: Html>(cx: Scope) -> View<G> {
    let state_input_text: &Signal<InputValue> = use_context(cx);
    let state_output_number: &Signal<OutputValue> = use_context(cx);
    view! {cx,
        div(class="h-14") {
            div(class="flex justify-end") {
                p(class="pb-2 text-xl font-semibold") { (state_output_number.get().value()) }
            }
            div {
                p { (state_input_text.get().value()) }
            }
        }
    }
}
