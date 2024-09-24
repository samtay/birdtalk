use dioxus::prelude::*;

use crate::{
    pack::{Pack, PackIdentifier},
    ui::{
        game::{GameView, GameViewPlaceholder},
        AppCtx,
    },
};

pub static PLAY_STATUS: GlobalSignal<Option<Pack>> = Signal::global(|| None);

#[component]
pub fn Play(pack_id: PackIdentifier) -> Element {
    // Do I need reactivity on pack_id? https://docs.rs/dioxus-hooks/0.6.0-alpha.2/dioxus_hooks/fn.use_effect.html#with-non-reactive-dependencies
    let pack_id = use_hook(|| CopyValue::new(pack_id));

    // Typically PLAY_STATUS is already loaded with the proper birdpack (if a user has navigated to
    // this route from within the app).
    let pack_to_play = use_memo(move || {
        PLAY_STATUS
            .read()
            .as_ref()
            .filter(|p| p.id == *pack_id.read())
            .cloned()
    });
    let mut error = use_signal(|| None);

    // But if not (perhaps a fresh page load on this route),
    let ctx = use_context::<AppCtx>();
    use_effect(move || {
        if pack_to_play.read().is_none() {
            spawn(async move {
                let pack_id = pack_id.read();
                let stats = ctx.stats.read();

                // Enforce ad-hoc review rounds are based on birds already learned.
                if let PackIdentifier::Birds(ref birds_requested) = *pack_id {
                    for bird_id in birds_requested {
                        if stats
                            .bird_stats()
                            .get(bird_id)
                            .filter(|b| b.learned)
                            .is_none()
                        {
                            error.set(Some(
                                "You haven't learned some of these birds yet!".to_string(),
                            ));
                            return;
                        }
                    }
                }

                let result = Pack::fetch_by_id(&pack_id).await;
                match result {
                    Ok(pack) => *PLAY_STATUS.write() = Some(pack),
                    Err(e) => error.set(Some(format!("{e}"))),
                }
            });
        }
    });

    match (pack_to_play(), error()) {
        (Some(pack), _) => rsx! { GameView { pack } },
        (_, Some(error)) => rsx! { ErrorView { error_msg: "{error}" } },
        _ => rsx! { GameViewPlaceholder {}},
    }
}

#[component]
fn ErrorView(error_msg: String) -> Element {
    rsx! {
        div {
            class: "text-red-dark text-center flex flex-col items-center justify-center gap-6 mb-auto mt-4",
            div { class: "text-3xl", "Uh oh! 😱" }
            div {
                class: "text-lg",
                "{error_msg}"
            }
        }
    }
}
