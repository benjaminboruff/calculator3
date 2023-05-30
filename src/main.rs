use sycamore::prelude::*;
fn main() {
    sycamore::render(|cx| {
        view! {
            cx,
            div(class="container mx-auto") {
                p(class="text-xl") { "Hello, World!"}
            }
        }
    })
}
