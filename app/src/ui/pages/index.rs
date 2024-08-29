use dioxus::prelude::*;

use crate::ui::components::PackOfTheDay;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "m-auto grid grid-cols-5",
            div {
                class: "col-span-5 sm:col-span-3 flex flex-col justify-between p-2 sm:p-6 gap-5",
                div {
                    class: "text-4xl text-center uppercase",
                    "10 new birds every day"
                }
                PackOfTheDay { }
            }
            div {
                // TODO: px-1/3 i think should work? check tailwind docs when internet works
                class: "col-span-5 sm:col-span-2 text-5xl text-left bg-red text-bold p-6 h-full sm:px-[33%]",
                class: "flex flex-col justify-center uppercase",
                "a game that helps you memorize bird calls"
            }
            div {
                class: "text-5xl col-span-5 text-left bg-yellow-dark text p-6 sm:p-16",
                span {
                    class: "text-5xl",
                    "Expert birders know "
                }
                span {
                    class: "text-3xl",
                    "the best way to find a bird is to hear it first. Practice here, and see you out there."
                }
            }
        }
    }
}
