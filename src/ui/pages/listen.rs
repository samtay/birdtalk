use dioxus::prelude::*;

#[component]
pub fn Listen() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-full w-full",
            h1 { "Listen mode!" }
            p { "Coming soon!" }
        }
    }
}
