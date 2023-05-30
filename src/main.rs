use components::keypad::Keypad;
use components::output::Output;
use sycamore::prelude::*;

mod components;

#[derive(Clone, Copy, PartialEq, Eq)]
struct InputValue(&'static str);
impl InputValue {
    fn value(self) -> &'static str {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq)]
struct OutputValue(f64);
impl OutputValue {
    fn value(self) -> f64 {
        self.0
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // Global state setup with text input and numeric result values
    let input_text = create_signal(cx, InputValue(""));
    provide_context_ref(cx, input_text);
    let output_number = create_signal(cx, OutputValue(0.0));
    provide_context_ref(cx, output_number);

    input_text.set(InputValue("Foobar"));
    view! {
        cx,
        div(class="container mx-auto") {
            div(class="flex flex-col gap-6") {
                div(class="p-2") {
                    p(class="text-xl") { "Calculator"}
                }
                div(class="p-4") {
                    Output {}
                }
                div(class="p-4") {
                    Keypad {}
                }
            }
        }
    }
}

fn main() {
    sycamore::render(|cx| view! {cx, App()});
}
