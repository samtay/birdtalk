use std::rc::Rc;

use dioxus::prelude::*;

use super::{quiz::BirdContext, GameCtx};
use crate::{bird::Bird, stats::LEARN_THRESHOLD};

#[component]
pub fn MultipleChoiceCard(bird: MappedSignal<BirdContext>, correct: bool) -> Element {
    let game_ctx = use_context::<GameCtx>();
    let bird_copy = bird.clone();
    let bird_memo = use_memo(move || bird_copy.read().bird.clone());
    let correct_chosen = game_ctx.correct_chosen;
    rsx! {
        div {
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
    // NOTE: this is assuming a different set of birds each round!
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
                    mistakenly_chosen.set(true);
                }
                game_ctx.record_choice(correct);
            },
            class: "group w-full h-full mx-auto border border-black rounded-xl shadow sm:enabled:hover:shadow-lg sm:enabled:hover:bg-yellow sm:enabled:hover:-translate-y-2 transition-transform bg-yellow-light focus:outline-none focus-visible:ring focus-visible:ring-yellow-dark disabled:shadow-none disabled:border disabled:opacity-50 disabled:transition-opacity disabled:duration-1000",
            class: "p-2 sm:p-4 flex flex-row sm:flex-col space-between items-center gap-1 sm:gap-4",
            class: if mistakenly_chosen() { "animate-shake" },
            disabled: mistakenly_chosen() || correct_chosen(),
            img {
                class: "border block w-20 h-20 sm:w-28 sm:h-28 rounded-full object-cover",
                src: bird.read().image_url(),
                alt: "",
            }
            div {
                class: "grow justify-center flex flex-col text-center",
                span {
                    class: "text-lg font-semibold",
                    // "American Three-toed Woodpecker"
                    "{bird().common_name}"
                }
                span {
                    class: "text-sm font-medium whitespace-nowrap text-ellipsis overflow-hidden",
                    // "Campylorhynchus brunneicapillus"
                    "{bird().scientific_name}"
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
            spawn(async move {
                if let Some(btn) = next_button.read().as_ref() {
                    btn.set_focus(true).await.ok();
                }
            });
        }
    });
    rsx! {
        div {
            class: "w-full h-full mx-auto border-green-dark rounded-xl shadow bg-green-light border-2",
            class: "p-2 sm:p-4 flex flex-row sm:flex-col space-between items-center gap-1",
            img {
                class: "border block w-20 h-20 sm:w-28 sm:h-28 rounded-full object-cover",
                src: bird.read().bird.image_url(),
                alt: "",
            }
            div {
                class: "grow flex flex-col text-center justify-between",
                div {
                    class: "text-lg font-semibold whitespace-nowrap",
                    // "American Three-toed Woodpecker"
                    "{bird().bird.common_name}"
                }
                BirdProgress { bird: bird.clone() }
                button {
                    class: "mt-2 px-2 py-1 focus:outline-none focus-visible:ring-2 focus-visible:ring-green-extra-dark font-semibold text-sm sm:text-base bg-green-dark text-white rounded-xl shadow sm:hover:scale-[1.05] sm:hover:shadow-xl transition-transform",
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
                    "Continue"
                }
            }
        }
    }
}

#[component]
fn BirdProgress(bird: MappedSignal<BirdContext>) -> Element {
    let total = LEARN_THRESHOLD;
    let progress = bird.read().consecutively_identified;
    rsx! {
        div {
            class: "flex gap-0 items-center justify-center",
            {(0..total).map(|ix| {
                rsx! {
                    div {
                        class: "w-4 h-4 rounded-full grow-0",
                        class: if ix < progress { "bg-green-dark" } else { "bg-offwhite" }
                    }
                    if ix + 1 < total {
                        div {
                            class: "w-8 h-[0.2rem]",
                            class: if ix + 1 < progress { "bg-green-dark" } else { "bg-offwhite" }
                        }
                    }
                }
            })}
        }
    }
}

#[component]
pub fn MultipleChoiceCardPlaceholder(ix: usize) -> Element {
    let common_name_width_class = match ix {
        0 => "w-32",
        1 => "w-40",
        2 => "w-24",
        _ => "w-28",
    };
    let scientific_name_width_class = match ix {
        0 => "w-40",
        1 => "w-48",
        2 => "w-36",
        _ => "w-32",
    };
    rsx! {
        div {
            class: "w-80 h-24 sm:w-64 sm:h-56 mx-auto bg-offwhite-2 border border-black/10 rounded-xl",
            class: "p-2 sm:p-4 flex flex-row sm:flex-col space-between items-center gap-2 sm:gap-4",
            // img
            div { class: "bg-black/10 block w-20 h-20 sm:w-28 sm:h-28 rounded-full" }
            div {
                class: "grow flex flex-col justify-center items-center gap-4",
                div {
                    class: "h-2.5 bg-black/20 rounded-full {common_name_width_class}"
                }
                div {
                    class: "h-2 w-28 bg-black/10 rounded-full {scientific_name_width_class}"
                }
            }
        }
    }
}
