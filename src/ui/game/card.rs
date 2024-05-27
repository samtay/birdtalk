use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
    bird::Bird,
    game::{BirdContext, Game},
};

#[component]
pub fn MultipleChoiceCard(
    bird: MappedSignal<BirdContext>,
    correct: bool,
    game: Signal<Game>,
    correct_chosen: Signal<bool>,
) -> Element {
    let bird_copy = bird.clone();
    let bird_memo = use_memo(move || bird_copy.read().bird.clone());
    let next_button_enabled = use_memo(move || *correct_chosen.read() && correct);
    rsx! {
        div {
            // TODO: try removing this with the other cubic, it might be better fitting vibe.
            class: "[perspective:1000px]",
            div {
                class: "grid transition-transform duration-500 [transform-style:preserve-3d] h-full",
                class: if correct && correct_chosen() {
                    "[transform:rotateY(180deg)]"
                },
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(0deg)]",
                    CardFront {
                        bird: bird_memo,
                        onclick: move |_| {
                            if correct {
                                correct_chosen.set(true);
                            }
                            modify_choice_stats(correct, game);
                        },
                        correct,
                        correct_chosen,
                    }
                }
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(-180deg)]",
                    CardBack {
                        bird,
                        onclick: move |_| {
                            if correct {
                                correct_chosen.set(false);
                                game.write().set_next_challenge();
                                tracing::info!(
                                    "set! new bird is: {:?}",
                                    game.read().correct_choice().bird.common_name
                                );
                            } else {
                                tracing::error!("This shouldn't happen. How did you get here?");
                            }
                        },
                        next_button_enabled
                    }
                }
            }
        }
    }
}

#[component]
fn CardFront(
    bird: Memo<Bird>,
    correct: bool,
    onclick: EventHandler<MouseEvent>,
    correct_chosen: Signal<bool>,
) -> Element {
    // TODO: note that this is assuming a different set of birds each round!
    let mut mistakenly_chosen = use_signal(|| false);
    use_effect(move || {
        bird.read();
        mistakenly_chosen.set(false);
    });
    rsx! {
        button {
            onclick: move |e| async move {
                // Handle mistaken state
                if !correct {
                    tracing::debug!("Setting mistakenly_chosen to true");
                    mistakenly_chosen.set(true);
                }
                // Let the parent know the choice was made
                onclick.call(e);
            },
            class: "group p-4 w-full h-full mx-auto border-amber-200 rounded-xl shadow enabled:hover:shadow-lg enabled:hover:bg-amber-200 space-y-2 bg-amber-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-600 focus-visible:ring-offset-2 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 disabled:shadow-none",
            class: if mistakenly_chosen() {
                "animate-shake"
            },
            class: if mistakenly_chosen() || correct_chosen() {
                "disabled border opacity-50 transition-opacity duration-1000"
            } else {
                "border-2"
            },
            disabled: mistakenly_chosen() || correct_chosen(),
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().img_file.to_string_lossy().to_string(),
                alt: bird().common_name,
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
fn CardBack(
    bird: MappedSignal<BirdContext>,
    onclick: EventHandler<MouseEvent>,
    next_button_enabled: Memo<bool>,
) -> Element {
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
            class: "p-4 w-full h-full mx-auto border-green-200 rounded-xl shadow space-y-2 bg-green-100/50 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 border-2",
            img {
                class: "animate-[spin_1s_linear] block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().bird.img_file.to_string_lossy().to_string(),
                alt: bird().bird.common_name,
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
                        class: "mt-2 px-4 py-2 focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-600 focus-visible:ring-offset-2 font-semibold text-sm sm:text-base bg-green-500 text-amber-50 rounded-full shadow-sm",
                        onclick: move |e| async move {
                            onclick.call(e);
                        },
                        onmounted: move |e| async move {
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

// TODO: most of this should live on the game itself
fn modify_choice_stats(correct: bool, game: Signal<Game>) {
    tracing::info!("handle_choice was called");
    let mut game = game;
    let mut game = game.write();
    let choice = game.correct_choice_mut();
    if correct {
        choice.identified += 1;
        choice.consecutively_identified += 1;
    } else {
        choice.mistaken += 1;
        choice.consecutively_identified = 0;
    }
}
