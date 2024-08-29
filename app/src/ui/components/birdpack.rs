use dioxus::prelude::*;

use crate::{
    bird::{BirdPack, BirdPackDaily},
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
    let mut position = use_signal(|| 0usize);
    let pos = move |ix: usize| (ix + pack_size - position()) % pack_size;
    let current = move |ix: usize| pos(ix) == 0;
    let next = move |ix: usize| pos(ix) == 1;
    let last = move |ix: usize| pos(ix) == pack_size - 1;

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
                div {
                    class: "w-56 h-96 relative",
                    for (ix, bird) in pack.birds.iter().enumerate() {
                        div {
                            key: ix,
                            class: "absolute inset-0 border-2 border-offwhite-2 rounded-xl shadow py-3 sm:py-4 text-black {bg_color(ix)} flex flex-row justify-between transition-transform transform-gpu duration-700 origin-bottom select-none",
                            // NOTE: this overwrites transform-gpu :/ I could make another closure
                            // to compute hardcoded transform strings, so that its tailwind all the way down.
                            transform: "rotate({degree(pos(ix))}deg) translateX({degree(pos(ix))}px)",
                            z_index: "{pack_size - pos(ix)}",
                            "data-position": "{pos(ix)}",

                            class: if current(ix) {
                                "select-text will-change-transform"
                            },

                            class: if next(ix) {
                                "z"
                            },

                            class: if last(ix) {
                                "animate-card-slide-out"
                            },

                            div {
                                class: "uppercase max-h-full self-end whitespace-nowrap text-ellipsis",
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
                                class: "uppercase max-h-full self-start whitespace-nowrap text-ellipsis",
                                text_orientation: "upright",
                                writing_mode: "vertical-lr",
                                "{bird.scientific_name.split_whitespace().last().unwrap()}"
                            }
                        }
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

// This may be useful in the future
mod dead_code {
    pub use super::*;

    #[component]
    fn PackOfTheDayOld() -> Element {
        let BirdPackDaily { pack, day: _ } = use_resource(BirdPackDaily::fetch_today)
            .suspend()?
            .read()
            .clone()?;
        rsx! {
            div {
                class: "flex-col gap-4 justify-between inline-flex sm:h-64 border-2 rounded-xl shadow p-3 sm:p-4 text-black bg-offwhite hover:bg-yellow-light hover:shadow-xl select-none relative items-center",
                div {
                    class: "text-lg font-semibold text-center",
                    "Daily Bevy" // "{pack.name}"
                }
                div {
                    class: "flex-initial sm:flex-none overflow-hidden",
                    div {
                        class: "grid grid-cols-5 gap-2.5 min-w-52 justify-items-center",
                        if pack.birds.is_empty() {
                            for _ in 0..10 {
                                span {class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full flex-none bg-purple"}
                            }
                        } else {
                            for bird in pack.birds.iter().take(10) {
                                // TODO: tooltip with common name
                                img {
                                    class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full object-cover flex-none max-sm:min-w-8 max-sm:min-h-8 sm:min-w-9 sm:min-h-9 overflow-hidden",
                                    src: bird.image_url(),
                                    alt: bird.common_name.clone(),
                                }
                            }
                        }
                    }
                }
                button {
                    class: "px-8 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-full shadow uppercase",
                    onclick: move |_| {
                        *PLAY_STATUS.write() = Some(pack.clone());
                        navigator().push(Route::Play { pack_id: pack.id });
                    },
                    "play"
                }
            }
        }
    }

    /// A grid of all the free bird packs to choose from
    #[component]
    pub fn BirdPackSelector(selected_pack: Signal<Option<BirdPack>>) -> Element {
        let packs_resource = use_resource(BirdPack::fetch_free_packs);
        let packs_value = packs_resource.value();
        let packs_read = packs_value.read();
        match *packs_read {
            None => rsx! {PackSelectorPlaceholder {}},
            Some(Err(ref e)) => rsx! {"TODO handle errors gracefully: {e}"},
            Some(Ok(ref packs)) => rsx! {
                fieldset {
                    legend {
                        class: "font-semibold text-2xl text-center mb-2",
                        "Pick a bird pack"
                    }
                    ul {
                        class: "grid grid-cols-1 sm:grid-cols-3 w-full gap-2 sm:gap-6 items-stretch",
                        for pack in packs.clone() {
                            BirdPackOption {pack, selection: selected_pack}
                        }
                    }
                }
            },
        }
    }

    /// Low effort placeholder (just a looading icon)
    #[component]
    fn PackSelectorPlaceholder() -> Element {
        rsx! {
            div {
                class: "mt-12 flex flex-col items-center justify-center",
                div {
                    class: "animate-spin w-24 h-24 border-t-4 border-green-800 rounded-full"
                }
            }
        }
    }

    /// A bird pack radio input option, styled as a card
    #[component]
    fn BirdPackOption(pack: BirdPack, selection: Signal<Option<BirdPack>>) -> Element {
        rsx! {
            li {
                label {
                    r#for: pack.id as i64,
                    class: "sm:flex-col gap-4 justify-between inline-flex h-full w-full border-2 rounded-xl shadow p-3 sm:p-4 transition-transform has-[:enabled]:hover:-translate-y-2 has-[:enabled]:hover:bg-yellow-light has-[:enabled]:hover:shadow-xl has-[:disabled]:opacity-50 focus-within:ring-2 focus-within:ring-purple-dark has-[:checked]:bg-purple-light has-[:checked]:has-[:enabled]:hover:bg-purple-light has-[:checked]:text-black cursor-pointer select-none relative",
                    input {
                        class: "absolute opacity-0 peer",
                        name: "pack",
                        id: pack.id as i64,
                        value: pack.id as i64,
                        r#type: "radio",
                        checked: selection.as_ref().filter(|bp| bp.id == pack.id).map(|_|true),
                        // TODO: onmount should probably be replaced with use_effect if its not using mount data?
                        // TODO: join with user data to send packs down with a "default choice" depending on where a user is at
                        onmounted: {
                            let pack = pack.clone();
                            move |_| {
                                if pack.name == "Common I" {
                                    tracing::debug!("onmount: setting pack to {:?}", pack.id);
                                    *selection.write() = Some(pack.clone())
                                }
                            }
                        },
                        onchange: {
                            let pack = pack.clone();
                            move |_| {
                                tracing::debug!("onchange: setting pack to {:?}", pack.id);
                                *selection.write() = Some(pack.clone());
                        }}
                    }
                    svg {
                        class: "w-6 h-6 text-purple-dark inline-block absolute right-2 top-2 invisible sm:peer-checked:visible",
                        view_box: "0 0 24 24",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            d: "M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            stroke_linecap: "round",
                            stroke_linejoin: "round"
                        }

                    }
                    svg {
                        class: "w-6 h-6 text-black inline-block absolute right-2 top-2 invisible sm:visible peer-checked:invisible",
                        view_box: "0 0 24 24",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        stroke_width: "1.5",
                        stroke: "currentColor",
                        path {
                            d: "M9 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z",
                            stroke_linecap: "round",
                            stroke_linejoin: "round"
                        }

                    }
                    div {
                        class: "text-lg font-semibold",
                        "{pack.name}"
                    }
                    div {
                        class: "flex-initial sm:flex-none overflow-hidden w-1/2 sm:w-full",
                        div {
                            class: "flex gap-1 sm:grid sm:grid-cols-5 sm:gap-2.5 sm:min-w-52",
                            if pack.birds.is_empty() {
                                for _ in 0..10 {
                                    span {class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full flex-none bg-purple"}
                                }
                            } else {
                                for bird in pack.birds.iter().take(10) {
                                    // TODO: tooltip with common name
                                    img {
                                        class: "max-sm:w-8 max-sm:h-8 sm:w-9 sm:h-9 rounded-full object-cover flex-none max-sm:min-w-8 max-sm:min-h-8 sm:min-w-9 sm:min-h-9 overflow-hidden",
                                        src: bird.image_url(),
                                        alt: bird.common_name.clone(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
