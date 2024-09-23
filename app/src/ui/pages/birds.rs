use std::collections::{HashSet, VecDeque};

use dioxus::prelude::*;

use crate::{
    bird::Bird,
    pack::{Pack, PackIdentifier},
    ui::{
        components::{
            icons::{CheckedCircle, UncheckedCircle},
            BirdCard, BirdCardPlaceholder,
        },
        pages::PLAY_STATUS,
        AppCtx, Route,
    },
};

// TODO: save these settings in local storage
static SIMULTANEOUS_CALLS: GlobalSignal<usize> = Signal::global(|| 1);
static LOOP_AUDIO: GlobalSignal<bool> = Signal::global(|| true);
const MINIMUM_BIRDS: usize = 10;

#[derive(Clone, Copy)]
struct AviaryCtx {
    /// Selected birds for review
    selected: Signal<HashSet<Bird>>,
    /// Birds whose audio is currently playing
    playing: Signal<VecDeque<Bird>>,
    /// Read only list of learned birds
    /// This should pretty much always remain static, but might change if someone learns new birds
    /// in a different tab/window.
    bird_ids: Memo<Vec<u64>>,
    /// Whether there are enough birds to start a round
    enough_birds: Memo<bool>,
}

impl AviaryCtx {
    /// Initialize a new game context (and provide it to children).
    fn init() -> Self {
        let stats = use_context::<AppCtx>().stats;
        let bird_ids = use_memo(move || stats.read().birds_learned());
        let enough_birds = use_memo(move || bird_ids.read().len() >= MINIMUM_BIRDS);
        let selected = use_signal(HashSet::new);
        let playing = use_signal(VecDeque::new);
        use_context_provider(|| Self {
            selected,
            playing,
            bird_ids,
            enough_birds,
        })
    }
}

#[component]
pub fn Birds() -> Element {
    let ctx = AviaryCtx::init();

    // NOTE: SSG hydration is finicky. This hack allows page load not to freeze.
    let mut no_birds = use_signal(|| false);
    if generation() == 0 {
        needs_update();
    }
    if generation() == 1 {
        no_birds.set(ctx.bird_ids.read().is_empty());
    }

    if no_birds() {
        rsx! { EmptyNest {} }
    } else {
        rsx! {
            div {
                class: "flex flex-col sm:flex-row gap-4 p-4 sm:px-8 sm:pt-2 sm:pb-0 h-full",
                Sidebar {}
                BirdCollection {}
            }
        }
    }
}

#[component]
fn EmptyNest() -> Element {
    rsx! {
        div {
            class: "text-center flex flex-col items-center justify-center gap-6 mt-8",
            div { class: "text-3xl", "An empty nest 🪹" }
            div {
                class: "text-lg",
                span {
                    "You need to learn more birds to fill out your aviary!"
                }
            }
            div {
                class: "text-lg",
                span {
                    "Play the "
                }
                PackOfTheDayLink {}
                span {
                    " to get started!"
                }
            }
        }
    }
}

#[component]
fn Sidebar() -> Element {
    let AviaryCtx {
        selected,
        enough_birds,
        ..
    } = use_context();
    let num_selected = use_memo(move || selected.read().len());
    let review_disabled = use_memo(move || num_selected() < MINIMUM_BIRDS);
    let select_to_review_text = use_memo(move || match num_selected() {
        0 => format!("Select {MINIMUM_BIRDS} birds to review"),
        x if x > 0 && x < MINIMUM_BIRDS - 1 => {
            format!("Select {} more birds", MINIMUM_BIRDS - x)
        }
        x if x == MINIMUM_BIRDS - 1 => "Select 1 more bird".to_string(),
        _ => "".to_string(),
    });

    rsx! {
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
                PackOfTheDayLink {}
                span {
                    " to learn more!"
                }
            }
            div {
                class: if enough_birds() {
                    "hidden"
                },
                "Once you've learned {MINIMUM_BIRDS} birds, come back here to play a round of review!"
            }
            div {
                class: "fixed bottom-0 left-0 right-0 z-10 p-2 pb-4 border-t bg-offwhite sm:static sm:mt-auto flex flex-col gap-2 items-center",
                class: if !enough_birds() {
                    "hidden"
                },
                span {
                    class: "text-center",
                    "{select_to_review_text}"
                }
                button {
                    class: "px-12 py-4 mt-2 border-2 border-green-extra-dark focus:outline-none focus-visible:ring focus-visible:ring-green-dark font-semibold text-base bg-green-dark text-white rounded-xl shadow sm:enabled:hover:shadow-xl sm:enabled:hover:scale-125 sm:enabled:hover:bg-gradient-to-r disabled:opacity-75 from-green to-green-dark transition-transform uppercase text-xl z-40",
                    disabled: review_disabled,
                    onclick: move |_| {
                        let birds = selected().into_iter().collect::<Vec<_>>();
                        let pack = Pack::from(birds);
                        let pack_id = pack.id.clone();
                        *PLAY_STATUS.write() = Some(pack);
                        navigator().push(Route::Play {pack_id});
                    },
                    "review"
                }
            }
        }
    }
}

#[component]
fn BirdCollection() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 w-full",
            // TODO: unhide and add controls for listening, sorting, etc.
            div {class: "hidden sticky top-0", "Some controls here etc."}
            div {BirdGrid {}}
        }
    }
}

// TODO: paginate! Use scroll events to load more birds.
// Can probably do something nice where birds being fetched are placeholder cards and then they fill in (maybe a hashmap of Options?)
#[component]
fn BirdGrid() -> Element {
    let bird_ids = use_context::<AviaryCtx>().bird_ids;

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

const BIRD_GRID_HEIGHT: &str = "sm:h-[calc(100vh-120px)]";

#[component]
fn BirdsInner(birds: Vec<Bird>) -> Element {
    // NOTE: might be better to use form values with a memo
    let AviaryCtx { enough_birds, .. } = use_context();
    rsx! {
        ul {
            tabindex: -1,
            class: "grid grid-cols-1 sm:grid-cols-[repeat(auto-fill,_minmax(14rem,_1fr))] gap-4 sm:gap-8 sm:overflow-auto {BIRD_GRID_HEIGHT} sm:pt-2 sm:pr-2",
            class: if enough_birds() {
                "mb-[8.25rem] sm:mb-0"
            },
            for bird in birds {
                BirdInner { bird }
            }
        }
    }
}

#[component]
fn BirdInner(bird: Bird) -> Element {
    let AviaryCtx {
        mut selected,
        enough_birds,
        ..
    } = use_context();
    let id = bird.id;
    let check_icon_class = if !enough_birds() { "hidden" } else { "" };
    let bird_card_class = if enough_birds() {
        "sm:hover:shadow-lg"
    } else {
        ""
    };
    rsx! {
        li {
            key: id,
            class: "flex justify-center",
            label {
                class: "relative w-full sm:w-56 sm:h-72",
                class: if enough_birds() { "cursor-pointer sm:hover:-translate-y-2 transition-transform" },
                input {
                    class: "absolute opacity-0 peer",
                    r#type: "checkbox",
                    disabled: !enough_birds(),
                    id: id as i64,
                    name: id as i64,
                    onchange: {
                        move |e: Event<FormData>| {
                            if e.data().checked() {
                                let inserted = selected.write().insert(bird.clone());
                                if !inserted {
                                    tracing::warn!("Bird {} was already selected! Form data: {:?}", bird.id, e.data());
                                }
                            } else {
                                let removed = selected.write().remove(&bird);
                                if !removed {
                                    tracing::warn!("Bird {} wasn't selected! Form data: {:?}", bird.id, e.data());
                                }
                            }
                        }
                    }
                }
                BirdCard {
                    bird: bird.clone(),
                    extra_classes: "w-full h-full bg-yellow shadow {bird_card_class} peer-checked:bg-green peer-checked:border-green-dark peer-checked:text-green-extra-dark peer-focus-visible:ring peer-focus-visible:ring-yellow-dark peer-checked:peer-focus-visible:ring-green-dark",
                    text_selection: false,
                }
                CheckedCircle {
                    extra_classes: "text-green-extra-dark inline-block absolute top-2 right-2 sm:top-auto sm:bottom-2 sm:right-[calc(50%-0.75rem)] invisible peer-checked:visible {check_icon_class}",
                }
                UncheckedCircle {
                    extra_classes: "inline-block absolute top-2 right-2 sm:top-auto sm:bottom-2 sm:right-[calc(50%-0.75rem)] peer-checked:invisible {check_icon_class}"
                }
            }
        }
    }
}

#[component]
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
            class: "animate-pulse grid grid-cols-1 sm:grid-cols-[repeat(auto-fill,_minmax(14rem,_1fr))] gap-4 sm:gap-8 sm:overflow-auto {BIRD_GRID_HEIGHT} sm:pt-2 sm:pr-2 mb-[8.25rem] sm:mb-0",
            for (ix, _id) in bird_ids.iter().enumerate() {
                BirdCardPlaceholder {
                    extra_classes: "sm:h-72 sm:w-56",
                    extra_scientific_first_class: height_first(ix),
                    extra_scientific_second_class: height_second(ix),
                }
            }
        }
    }
}

#[component]
fn PackOfTheDayLink() -> Element {
    rsx! {
        Link {
            class: "font-semibold underline text-purple-dark outline-none focus-visible:ring sm:hover:text-white sm:hover:bg-purple-dark",
            to: Route::Play { pack_id: PackIdentifier::default() },
            "Pack of the Day"
        }
    }
}
