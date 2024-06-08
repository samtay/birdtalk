use std::rc::Rc;

use dioxus::prelude::*;

use super::{quiz::BirdContext, GameCtx};
use crate::bird::Bird;

#[component]
pub fn MultipleChoiceCard(bird: MappedSignal<BirdContext>, correct: bool) -> Element {
    let game_ctx = use_context::<GameCtx>();
    let bird_copy = bird.clone();
    let bird_memo = use_memo(move || bird_copy.read().bird.clone());
    let correct_chosen = game_ctx.correct_chosen;
    rsx! {
        div {
            // TODO: try removing this with the other cubic, it might be better fitting vibe.
            class: "[perspective:1000px]",
            div {
                class: "grid transition-transform duration-500 [transform-style:preserve-3d] h-full",
                class: if correct && correct_chosen() { "[transform:rotateY(180deg)]" },
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(0deg)]",
                    CardFront {
                        bird: bird_memo,
                        correct,
                    }
                }
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(-180deg)]",
                    CardBack {
                        bird,
                        correct,
                    }
                }
            }
        }
    }
}

#[component]
fn CardFront(bird: Memo<Bird>, correct: bool) -> Element {
    let mut game_ctx = use_context::<GameCtx>();
    // TODO: note that this is assuming a different set of birds each round!
    let mut mistakenly_chosen = use_signal(|| false);
    let correct_chosen = game_ctx.correct_chosen;
    use_effect(move || {
        bird.read();
        mistakenly_chosen.set(false);
    });
    rsx! {
        button {
            onclick: move |_| {
                if !correct {
                    tracing::debug!("Setting mistakenly_chosen to true");
                    mistakenly_chosen.set(true);
                }
                game_ctx.record_choice(correct);
            },
            class: "group w-full h-full mx-auto border-2 border-amber-200 rounded-xl shadow enabled:hover:shadow-lg enabled:hover:bg-amber-200 enabled:hover:-translate-y-2 transition-transform space-y-2 bg-amber-50 focus:outline-none focus-visible:ring focus-visible:ring-amber-600 px-2 py-2 sm:py-4 sm:px-4 md:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 disabled:shadow-none",
            class: if mistakenly_chosen() { "animate-shake" },
            class: if mistakenly_chosen() || correct_chosen() { "disabled border opacity-50 transition-opacity duration-1000" },
            disabled: mistakenly_chosen() || correct_chosen(),
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().img_file,
                alt: bird().common_name
            }
            div {
                class: "text-center space-y-2 sm:text-left",
                div {
                    class: "space-y-0.5",
                    p {
                        class: "text-lg text-amber-950 font-semibold group-enabled:group-hover:text-green-800",
                        "{bird().common_name}"
                    }
                    p {
                        class: "text-sm sm:text-base text-slate-500 font-medium group-enabled:group-hover:text-green-800/75",
                        "{bird().scientific_name}"
                    }
                }
            }
        }
    }
}

#[component]
fn CardBack(bird: MappedSignal<BirdContext>, correct: bool) -> Element {
    let mut game_ctx = use_context::<GameCtx>();
    let next_button_enabled = use_memo(move || *game_ctx.correct_chosen.read() && correct);
    let mut next_button: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if next_button_enabled() {
            tracing::debug!("Spawning to focus next button...");
            spawn(async move {
                tracing::debug!("Trying to act on button...");
                if let Some(btn) = next_button.read().as_ref() {
                    tracing::debug!("Setting focus on next button");
                    btn.set_focus(true).await.ok();
                }
            });
        }
    });
    rsx! {
        div {
            class: "w-full h-full mx-auto border-green-200 rounded-xl shadow space-y-2 bg-green-100/50 px-2 py-2 sm:py-4 sm:px-4 md:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 border-2",
            img {
                class: "animate-[spin_1s_linear] block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().bird.img_file,
                alt: bird().bird.common_name
            }
            div {
                class: "text-center sm:text-left w-full",
                div {
                    class: "flex flex-col justify-between",
                    div {
                        class: "text-lg font-semibold text-green-800 whitespace-nowrap",
                        "Nice work!"
                    }
                    div {
                        class: "flex space-x-2 text-sm text-green-800/75 whitespace-nowrap",
                        div {
                            "Identified: {bird().identified}"
                        }
                        div {
                            "Streak: {bird().consecutively_identified}"
                        }
                    }
                    button {
                        class: "mt-2 px-4 py-2 focus:outline-none focus-visible:ring-2 focus-visible:ring-green-400 font-semibold text-sm sm:text-base bg-green-800 text-amber-50 rounded-full shadow",
                        onclick: move |_| async move {
                            if correct {
                                game_ctx.next().await;
                            } else {
                                tracing::error!("This shouldn't happen. How did you get here?");
                            }
                        },
                        onmounted: move |e| {
                            next_button.set(Some(e.data()));
                        },
                        disabled: !next_button_enabled(),
                        "Ok!"
                    }
                }
            }
        }
    }
}
