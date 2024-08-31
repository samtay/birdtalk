use dioxus::prelude::*;

use crate::{
    bird::{Bird, BirdPackDaily},
    ui::{components::icons::ArrowUturnRightIcon, Route, PLAY_STATUS},
};

/// Pack of the day
#[component]
pub fn PackOfTheDay() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |errors: ErrorContext| rsx! {
                div {
                    class: "text-red-dark",
                    div { "Drats! Something went wrong fetching today's challenge. Please open a GitHub issue with the following error:" }
                    code { "{errors:?}" }
                }
            },
            SuspenseBoundary {
                fallback: |_context: SuspenseContext| rsx! {
                    div {"some pretty pulsing loading pack!"}
                },
                PackOfTheDayInner {}
            }
        }
    }
}

// TODO: "Daily Bevy" in a half circle as title above?
// TODO: audio play options on each card
// TODO: arrow key shortcuts
// TODO: "Next pack in ..."
// TODO: favor ul/li over divs?
// TODO: rm noisy curr/next/last closures
#[component]
fn PackOfTheDayInner() -> Element {
    let BirdPackDaily { pack, day: _ } = use_resource(BirdPackDaily::fetch_today)
        .suspend()?
        .read()
        .clone()?;
    let pack_size = pack.birds.len();
    let mut position = use_signal(|| 0usize);

    rsx! {
        div {
            class: "grid grid-cols-5 items-center mx-auto overflow-x-clip sm:overflow-x-visible",
            button {
                class: "col-span-1 w-12 h-12 focus:outline-none focus-visible:ring focus-visible:ring-black font-semibold bg-offwhite text-black rounded-full shadow sm:hover:shadow-xl sm:hover:scale-110 flex justify-center items-center z-40 justify-self-end sm:justify-self-center order-last sm:order-first",
                onclick: move |_| {
                    position.with_mut(|p| *p = (*p + 1) % pack_size);
                },
                ArrowUturnRightIcon {}
            }
            div {
                class: "col-start-2 col-span-3 justify-self-stretch flex flex-col gap-6 items-center justify-center",
                ul {
                    class: "w-56 h-96 relative",
                    for (ix, bird) in pack.birds.clone().into_iter().enumerate() {
                        Card { bird, ix, pack_size, position }
                    }
                }
                button {
                    class: "px-12 py-4 mt-2 focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-xl shadow sm:hover:shadow-xl sm:hover:scale-125 sm:hover:bg-gradient-to-r from-green to-green-dark transition-transform uppercase text-xl z-40",
                    onclick: move |_| {
                        *PLAY_STATUS.write() = Some(pack.clone());
                        navigator().push(Route::Play { pack_id: pack.id });
                    },
                    "play"
                }
            }
        }
    }
}

/// Render the individual cards in the daily pack component.
/// - `ix` is the index of the bird in the pack, which does not change.
/// - `pack_size` is the total number of birds in the pack.
/// - `position` is the index of the card from the user's perspective.
///    This changes when the user clicks to view the next card.
#[component]
fn Card(bird: Bird, ix: usize, pack_size: usize, position: Signal<usize>) -> Element {
    let degree = |pos: usize| match pos {
        0 => 0,
        1 => 5,
        2 => 8,
        3 => 10,
        i => i + 7,
    };
    let bg_color = |ix: usize| match ix % 8 {
        0 => "bg-green",
        1 => "bg-yellow",
        2 => "bg-blue-light",
        3 => "bg-orange",
        4 => "bg-purple",
        5 => "bg-red",
        6 => "bg-chartreuse",
        7 => "bg-pink",
        _ => unreachable!(),
    };
    let pos = use_memo(move || (ix + pack_size - position()) % pack_size);
    rsx! {
        li {
            key: ix,
            class: "absolute inset-0 border-2 border-offwhite-2 rounded-xl shadow py-3 sm:py-4 text-black {bg_color(ix)} flex flex-row justify-between transition-transform transform-gpu duration-700 origin-bottom select-none",
            // NOTE: this overwrites transform-gpu :/ I could make another closure
            // to compute hardcoded transform strings, so that its tailwind all the way down.
            transform: "rotate({degree(pos())}deg) translateX({degree(pos())}px)",
            z_index: "{pack_size - pos()}",
            "data-position": "{pos()}",

            // current
            class: if pos() == 0 {
                "select-text will-change-transform"
            },

            // next
            class: if pos() == 1 {
                "z"
            },

            // last
            class: if pos() == pack_size - 1 {
                "animate-card-slide-out"
            },

            div {
                class: "uppercase max-h-full self-end whitespace-nowrap text-ellipsis overflow-hidden",
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().next().unwrap()}"
            }

            // center
            div {
                class: "flex flex-col gap-4 items-center",
                img {
                    class: "w-24 h-24 rounded-full object-cover flex-none overflow-hidden",
                    src: bird.image_url(),
                    alt: "{bird.common_name}",
                }
                div {
                    class: "text-lg text-center select-all",
                    "{bird.common_name}"
                }
            }

            div {
                class: "uppercase max-h-full self-start whitespace-nowrap text-ellipsis overflow-hidden",
                text_orientation: "upright",
                writing_mode: "vertical-lr",
                "{bird.scientific_name.split_whitespace().last().unwrap()}"
            }
        }
    }
}
