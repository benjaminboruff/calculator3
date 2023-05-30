use super::key::Key;
use sycamore::prelude::*;

#[component]
pub fn Keypad<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div {
            "Keypad component"
            Key {}
        }
    }
}
