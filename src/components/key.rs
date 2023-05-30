use sycamore::prelude::*;

#[derive(Props)]
pub struct KeyProps {
    key: &'static str,
    value: &'static str,
}

#[component]
pub fn Key<G: Html>(cx: Scope, props: KeyProps) -> View<G> {
    view! {cx,
        div(id=props.key, class="bg-blue-200 rounded-lg") { (props.value) }
    }
}
