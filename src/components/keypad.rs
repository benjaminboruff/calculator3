use super::key::Key;
use sycamore::prelude::*;

#[component]
pub fn Keypad<G: Html>(cx: Scope) -> View<G> {
    let ROW1: Vec<&str> = vec!["1", "2", "3", "+"];
    let ROW2: Vec<&str> = vec!["4", "5", "6", "-"];
    let ROW3: Vec<&str> = vec!["7", "8", "9", "*"];
    let ROW4: Vec<&str> = vec!["0", ".", "blank", "/"];

    let row1KeySetView = View::new_fragment(
        ROW1.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val){} })
            .collect(),
    );
    let row2KeySetView = View::new_fragment(
        ROW2.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val){} })
            .collect(),
    );
    let row3KeySetView = View::new_fragment(
        ROW3.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val){} })
            .collect(),
    );
    let row4KeySetView = View::new_fragment(
        ROW4.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val){} })
            .collect(),
    );

    view! {cx,
        div(class="m-4 bg-gray-300 rounded-md") {
            div(class="p-4 grid grid-cols-4 gap-2") {
                (row1KeySetView)
                (row2KeySetView)
                (row3KeySetView)
                (row4KeySetView)
            }

        }
    }
}
