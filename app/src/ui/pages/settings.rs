use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-full w-full",
            h1 { "Settings!" }
            p { "Coming soon!" }
        }
    }
}
