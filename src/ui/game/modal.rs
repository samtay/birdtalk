use dioxus::prelude::*;

use crate::game::{BirdContext, Game};

// TODO: onclick handler for dismissing modal
#[component]
fn BirdDetailsModal(
    bird_context: Memo<BirdContext>,
    game: Signal<Game>,
    visible: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "h-screen w-screen fixed top-0 left-0 z-50 block bg-amber-400/50 overflow-y-hidden",
            class: if visible() {
                "visible opacity-100 z-100 transition-opacity scale-100"
            } else {
                "invisible opacity-0 z-0 invisible scale-110"
            },
            div {
                class: "h-full max-w-screen-md mx-4 md:mx-auto flex flex-col justify-center items-center",
                BirdDetails { bird_context, game, visible }
            }
        }
    }
}

// TODO: just pass handler rather than game, visible
#[component]
fn BirdDetails(
    bird_context: Memo<BirdContext>,
    game: Signal<Game>,
    visible: Signal<bool>,
) -> Element {
    rsx! {
        // TODO: change styles or de-dupe from card
        div {
            class: "animate-fly-in overflow-hidden w-20 h-20 rounded-full group p-4 mx-auto border-2 border-amber-200 shadow enabled:hover:shadow-lg enabled:hover:bg-amber-200 space-y-2 bg-amber-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-600 focus-visible:ring-offset-2 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 disabled:shadow-none",
            // start class: "w-20 h-20 rounded-full animate-fly-in opacity-0",
            // eventually class: w-full h-96 rounded-xl opacity-100
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird_context().bird.img_file.to_string_lossy().to_string(),
                alt: bird_context().bird.common_name,
            }
            div {
                class: "text-center space-y-2 sm:text-left",
                div {
                    class: "space-y-0.5",
                    p {
                        class: "text-lg text-amber-950 font-semibold group-enabled:group-hover:text-green-800",
                        "{bird_context().bird.common_name}"
                    }
                    p {
                        class: "text-sm sm:text-base text-slate-500 font-medium group-enabled:group-hover:text-green-800/75",
                        "{bird_context().bird.scientific_name}"
                    }
                }
            }
            div {
                class: "grid grid-cols-1 gap-2",
                div {
                    class: "text-sm text-slate-500 font-medium",
                    "Identified: {bird_context().identified}"
                }
                div {
                    class: "text-sm text-slate-500 font-medium",
                    "Current streak: {bird_context().consecutively_identified}"
                }
            }
            button {
                class: "rounded-lg bg-green-800 text-amber-50",
                onclick: move |_| async move {
                    visible.set(false);
                    game.write().set_next_challenge();
                },
                "Got it!"
            }
        }
    }
}
