use components::keypad::Keypad;
use components::output::Output;
use sycamore::prelude::*;

mod components;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        div(class="container mx-auto") {
            div {
                p(class="text-xl") { "Calculator"}
            }
            div {
                Output {}
            }
            div {
                Keypad {}
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! {cx, App()});
}
