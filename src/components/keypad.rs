use super::key::Key;
use super::output::Output;
use sycamore::prelude::*;

#[component]
pub fn Keypad<G: Html>(cx: Scope) -> View<G> {
    let ROW1: Vec<&str> = vec!["1", "2", "3", "+"];
    let ROW2: Vec<&str> = vec!["4", "5", "6", "-"];
    let ROW3: Vec<&str> = vec!["7", "8", "9", "*"];
    // let ROW4: Vec<&str> = vec!["0", ".", "", "/"];

    let row1KeySetView = View::new_fragment(
        ROW1.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val, attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} })
            .collect(),
    );
    let row2KeySetView = View::new_fragment(
        ROW2.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val, attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} })
            .collect(),
    );
    let row3KeySetView = View::new_fragment(
        ROW3.iter()
            .map(|&val| view! {cx, Key::<G>(key=val, value=val, attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} })
            .collect(),
    );
    let row4KeySetView = View::new_fragment(vec![
        view! {cx, Key::<G>(key="0", value="0", attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} },
        view! {cx, Key::<G>(key=".", value=".", attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} },
        view! {cx, Key::<G>(key="", value="", attr:class="bg-transparent rounded-md min-w-full"){} },
        view! {cx, Key::<G>(key="/", value="/", attr:class="bg-sky-700 rounded-md min-w-full text-lg text-center text-white shadow-md"){} },
    ]);

    let clearKey = View::new_fragment(vec![view! {cx,
        Key::<G>(key="clear", value="clear", attr:class="grow bg-red-600 rounded-md py-3 font-semibold text-center text-white shadow-md"){}
    }]);

    let undoKey = View::new_fragment(vec![view! {cx,
        Key::<G>(key="undo", value="undo", attr:class="grow bg-red-600 rounded-md py-3 font-semibold text-center text-white shadow-md"){}
    }]);

    let equalKey = View::new_fragment(vec![view! {cx,
        Key::<G>(key="=", value="=", attr:class="bg-green-600 rounded-md py-2 min-w-full text-3xl font-semibold text-center text-white shadow-md"){}
    }]);

    let emptyKey = View::new_fragment(vec![view! {cx,
        Key::<G>(key="", value="", attr:class="grow-0 bg-transparent"){}
    }]);

    view! {cx,
        div(class="shadow-lg") {
            div(class="flex flex-col") {
                div(class="p-8 bg-sky-700 rounded-t-lg") {
                    h4(class="text-center text-2xl text-white") { "Calculator"}
                }
                div(class="bg-neutral-200") {
                    div(class="bg-neutral-300 m-4 p-4 rounded-md shadow-inner") {
                        div(class="flex justify-end") {
                            Output {}
                        }
                    }
                }
                div(class="bg-neutral-200 rounded-b-lg") {
                    div(class="px-4 grid grid-cols-4 gap-1") {
                        (row1KeySetView)
                        (row2KeySetView)
                        (row3KeySetView)
                        (row4KeySetView)
                    }
                    div(class="m-4") { (equalKey) }
                    div(class="flex m-4 gap-1") {
                        (clearKey)
                        (emptyKey)
                        (undoKey)
                    }
                }
            }
        }
    }
}
