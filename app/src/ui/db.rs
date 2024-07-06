use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use crate::bird::BirdPack;

// TODO: There's some clever stuff in `use_server_future` that does auto caching on web?
// TODO: see https://docs.rs/dioxus/latest/dioxus/prelude/fn.use_resource.html
// Probably call that from _outside_ the router, hit supabase pg, store something in ctx,
// then write to the ctx when user selects bird pack on home screen.
// pub fn fetch_pack(id: String) -> Signal<BirdPack> {
//     if id != "demo" {
//         tracing::error!("Only demo pack is implemented!");
//     }
//     let storage_key = format!("pack-{id}");
//     // TODO: Probably need use_reactive on id?
//     // TODO: use_server_future(future) ?
//
//     // TODO: can get rid of this after dioxus-sdk fix to detect ssr/hydrate
//     if cfg!(feature = "server") {
//         use_signal(|| BirdPack {
//             id: "demo".to_string(),
//             birds: crate::bird::demo_data(),
//             already_learned: false,
//         })
//     } else {
//         use_synced_storage::<LocalStorage, _>(storage_key, || BirdPack {
//             id: "demo".to_string(),
//             birds: crate::bird::demo_data(),
//             already_learned: false,
//         })
//     }
// }

// #[server]
// async fn get_server_data() -> Result<String, ServerFnError> {
//     // Access a database
//     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//     Ok("Hello from the server!".to_string())
// }
