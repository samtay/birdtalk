use dioxus::prelude::*;

use crate::ui::components::PackOfTheDay;

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "m-auto grid grid-cols-10",
            div {
                class: "col-span-10 sm:col-span-6 lg:col-span-5 p-2 pb-4 sm:p-6",
                div {
                    // screen-lg == 1024px, 5/10 of the lg-screen is 512px
                    class: "lg:ml-auto lg:max-w-[512px] flex flex-col justify-between gap-5",
                    div {
                        class: "text-4xl text-center uppercase",
                        "10 new birds every day"
                    }
                    PackOfTheDay { }
                }
            }
            div {
                class: "col-span-10 sm:col-span-4 lg:col-span-5 bg-red p-8 h-full flex flex-col items-start sm:items-center justify-center",
                div {
                    // screen-lg == 1024px, 5/10 of the lg-screen is 512px
                    class: "lg:mr-auto lg:ml-48 lg:max-w-[512px] uppercase",
                    div {
                        class: "w-full sm:w-56 text-5xl text-left text-bold leading-normal sm:leading-tight",
                        "A game that helps you memorize bird calls."
                    }
                }
            }
            div {
                class: "text-5xl col-span-10 text-left bg-yellow-dark text p-8 sm:p-16",
                div {
                    class: "max-w-screen-lg mx-auto",
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
}
