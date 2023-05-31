use components::keypad::Keypad;
use components::output::Output;
use regex::Regex;
use sycamore::prelude::*;

mod components;

struct InputValue(String);
impl InputValue {
    fn value(&self) -> &String {
        &self.0
    }

    fn is_valid(&self) -> bool {
        validate_arithmetic(&self.0)
    }
}

struct OutputValue(f64);
impl OutputValue {
    fn value(&self) -> f64 {
        self.0
    }
}

fn validate_arithmetic(math: &String) -> bool {
    let re = Regex::new(r"^\s*\d+(\.\d+)?\s*[+-/*/]\s*\d+(\.\d+)?\s*(\s*([+-/*/])\s*\d+(\.\d+)?)*\s*$")
        .unwrap();

    re.is_match(&math)
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // Global state setup with text input and numeric result values
    let input_text = create_signal(cx, InputValue("".to_string()));
    provide_context_ref(cx, input_text);
    let output_number = create_signal(cx, OutputValue(0.0));
    provide_context_ref(cx, output_number);

    input_text.set(InputValue("".to_string()));
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
