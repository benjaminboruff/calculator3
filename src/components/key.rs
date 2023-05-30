use sycamore::prelude::*;

#[component]
pub fn Key<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div { "Key component" }
    }
}
