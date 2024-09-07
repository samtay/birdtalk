use dioxus::prelude::*;

use crate::{bird::Bird, ui::AppCtx};

#[component]
pub fn Birds() -> Element {
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
        None => rsx! { h1 { "Your Aviary" } BirdsPlaceholder {bird_ids} },
        Some(Ok(birds)) => rsx! { h1 {"Your Aviary" } BirdsInner {birds: birds.clone()} },
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
    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
            for bird in birds {
                div {"{bird.common_name}"}
            }
        }
    }
}

#[component]
fn BirdsPlaceholder(bird_ids: ReadOnlySignal<Vec<u64>>) -> Element {
    rsx! {
        div {
            class: "animate-pulse grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4",
            for _ in bird_ids.iter() {
                div {class: "h-2.5 w-2.5 bg-black/20 rounded-sm"}
            }
        }
    }
}
