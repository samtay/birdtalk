use dioxus::prelude::*;

use crate::{
    bird::BirdPack,
    ui::{
        game::{GameView, GameViewPlaceholder},
        PLAY_STATUS,
    },
};

#[component]
pub fn Play(pack_id: u64) -> Element {
    // Do I need reactivity on pack_id? https://docs.rs/dioxus-hooks/0.6.0-alpha.2/dioxus_hooks/fn.use_effect.html#with-non-reactive-dependencies

    // Typically PLAY_STATUS is already loaded with the proper birdpack (if a user has navigated to
    // this route from within the app).
    let pack_to_play = use_memo(move || {
        PLAY_STATUS
            .read()
            .as_ref()
            .filter(|p| p.id == pack_id)
            .cloned()
    });

    // But if not (perhaps a fresh page load on this route),
    use_effect(move || {
        if pack_to_play.read().is_none() {
            spawn(async move {
                // TODO: how to handle errors here?
                let pack = BirdPack::fetch_by_id(pack_id).await.unwrap();
                *PLAY_STATUS.write() = Some(pack);
            });
        }
    });

    let x = match pack_to_play.read().as_ref() {
        Some(pack) => rsx! { GameView { pack: pack.clone() } },
        _ => rsx! { GameViewPlaceholder {}},
    };
    x
}
