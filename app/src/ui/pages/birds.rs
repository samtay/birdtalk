use std::collections::{HashSet, VecDeque};

use dioxus::prelude::*;

use crate::{
    bird::Bird,
    ui::{
        components::{BirdCard, BirdCardPlaceholder},
        AppCtx, Route,
    },
};

static SIMULTANEOUS_CALLS: GlobalSignal<usize> = Signal::global(|| 1);
static LOOP_AUDIO: GlobalSignal<bool> = Signal::global(|| true);

#[component]
pub fn Birds() -> Element {
    rsx! {
        div {
            class: "flex flex-col sm:flex-row gap-4 p-4 sm:p-8 sm:pb-0",
            div {
                class: "text-center sm:text-left text-lg flex flex-col gap-4 sm:max-w-xs",
                h2 {
                    class: "text-3xl",
                    "Your Aviary"
                }
                div {
                    span {
                        "Here are all the birds you've learned so far! 🐦 Continue to play the "
                    }
                    Link {
                        class: "underline text-purple-dark",
                        // TODO: after switching to query param, this can be Route::Play
                        to: Route::Index {},
                        "Pack of the Day"
                    }
                    span {
                        " to learn more!"
                    }
                }
                div {
                    class: "fixed bottom-0 left-0 right-0 z-10 pt-2 pb-4 border-t bg-offwhite sm:static sm:mt-auto flex flex-col gap-2 items-center",
                    span { "Select 10 birds to review"}
                    button {
                        class: "px-12 py-4 mt-2 border-2 border-green-extra-dark focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-xl shadow sm:hover:shadow-xl sm:hover:scale-125 sm:hover:bg-gradient-to-r from-green to-green-dark transition-transform uppercase text-xl z-40",
                        onclick: move |_| {
                            tracing::info!("TODO review birds");
                            // navigator().push(Route::Play { pack_id: pack.id });
                        },
                        "review"
                    }
                }
            }
            BirdCollection {}
        }
    }
}

#[component]
fn BirdCollection() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 w-full",
            div {class: "sticky top-0", "Some controls here etc."}
            div {BirdGrid {}}
        }
    }
}

#[component]
fn BirdGrid() -> Element {
    let stats = use_context::<AppCtx>().stats;
    let bird_ids = use_memo(move || stats.read().birds_learned());

    // TODO: paginate! Use scroll events to load more birds.
    // Can probably do something nice where birds being fetched are placeholder cards and then they
    // fill in (maybe a hashmap of Options?)

    let birds =
        use_resource(
            move || async move { Bird::fetch_by_ids(bird_ids.read().iter().copied()).await },
        );

    match &*birds.read_unchecked() {
        None => rsx! { BirdsPlaceholder {bird_ids} },
        Some(Ok(birds)) => rsx! { BirdsInner {birds: birds.clone()} },
        Some(Err(e)) => rsx! {
            div {
                class: "text-red-dark text-center flex flex-col items-center justify-center gap-6 mb-auto",
                div { class: "text-3xl", "Uh oh! 😱" }
                div {
                    class: "text-lg",
                    span {
                        "Something went wrong fetching your birds! Please open a "
                    }
                    a {
                        class: "underline text-purple-dark",
                        href: "https://github.com/samtay/birdtalk/issues/new",
                        target: "_blank",
                        "GitHub issue"
                    }
                    span { " with the following error:" }
                }
                div {
                    code {
                        class: "select-all",
                        "{e}"
                    }
                }
            }
        },
    }
}

#[component]
fn BirdsInner(birds: Vec<Bird>) -> Element {
    // NOTE: might be better to use form values with a memo
    let mut birds_selected = use_signal(|| HashSet::<u64>::new());
    let mut birds_playing = use_signal(|| VecDeque::<u64>::new());
    rsx! {
        ul {
            class: "grid grid-cols-1 sm:grid-cols-[repeat(auto-fill,_minmax(14rem,_1fr))] gap-4 sm:gap-8 sm:overflow-auto sm:h-[calc(100vh-176px)] sm:pr-2 mb-[8.25rem] sm:mb-0",
            for bird in birds {
                li {
                    key: bird.id,
                    BirdCard { bird, extra_classes: "sm:h-72 sm:max-w-56 bg-yellow" }
                }
            }
        }
    }
}

#[component]
// TODO: update this to match the finished aviary design
fn BirdsPlaceholder(bird_ids: ReadOnlySignal<Vec<u64>>) -> Element {
    let height_first = |ix| match ix % 3 {
        0 => "h-40",
        1 => "h-32",
        _ => "h-48",
    };
    let height_second = |ix| match ix % 4 {
        0 => "h-48",
        1 => "h-36",
        2 => "h-44",
        _ => "h-40",
    };
    rsx! {
        div {
            class: "animate-pulse grid grid-cols-1 sm:grid-cols-[repeat(auto-fill,_minmax(14rem,_1fr))] gap-4 sm:gap-8 sm:overflow-auto sm:h-[calc(100vh-176px)] sm:pr-2 mb-[8.25rem] sm:mb-0",
            for (ix, _id) in bird_ids.iter().enumerate() {
                BirdCardPlaceholder {
                    extra_classes: "sm:h-72 sm:max-w-56",
                    extra_scientific_first_class: height_first(ix),
                    extra_scientific_second_class: height_second(ix),
                }
            }
        }
    }
}
