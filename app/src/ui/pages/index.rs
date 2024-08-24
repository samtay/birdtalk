use dioxus::prelude::*;

use crate::ui::components::PackOfTheDay;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "container max-w-screen-lg m-auto mt-2 px-2 grid grid-cols-5 gap-5",
            div {
                class: "col-span-5 text-2xl text-center place-self-center",
                "Welcome to BirdTalk!"
            }
            div {
                class: "col-span-5 text-lg text-center place-self-center",
                "An app that helps you memorize bird calls"
            }
            div {
                class: "col-span-3 text-lg",
                "Try the Daily Bevy! Packs reset at midnight."
            }
            div {
                class: "col-span-2",
                PackOfTheDay { }
            }
        }
    }
}
