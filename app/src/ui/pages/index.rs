use dioxus::prelude::*;

use crate::ui::components::PackOfTheDay;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "m-auto grid grid-cols-5",
            div {
                class: "col-span-5 sm:col-span-3 flex flex-col justify-between p-2 pb-4 sm:p-6 gap-5",
                div {
                    class: "text-4xl text-center uppercase",
                    "10 new birds every day"
                }
                PackOfTheDay { }
            }
            div {
                class: "col-span-5 sm:col-span-2 bg-red p-8 h-full flex flex-col justify-center uppercase items-start sm:items-center",
                div {
                    class: "w-full sm:w-56 text-5xl text-left text-bold leading-normal sm:leading-tight",
                    "A game that helps you memorize bird calls."
                }
            }
            div {
                class: "text-5xl col-span-5 text-left bg-yellow-dark text p-8 sm:p-16",
                span {
                    class: "text-5xl",
                    "The wild speaks. "
                }
                span {
                    class: "text-3xl",
                    "One of the best ways to spot a bird is to hear it first. Learn to recognize new calls here for your next adventure out in the field."
                }
            }
        }
    }
}
