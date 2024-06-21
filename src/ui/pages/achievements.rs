use dioxus::prelude::*;

#[component]
pub fn Achievements() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-full w-full",
            h1 { "Achievements!" }
            p { "Coming soon!" }
        }
    }
}
