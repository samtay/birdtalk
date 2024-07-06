use dioxus::prelude::*;

#[component]
pub fn Birds() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-full w-full",
            h1 { "Birds; or other types of collections?" }
            p { "Coming soon!" }
        }
    }
}
