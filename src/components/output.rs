use sycamore::prelude::*;

#[component]
pub fn Output<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        div {
            "Output component"
        }
    }
}
