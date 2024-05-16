#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use crate::bird::Bird;

mod bird;

// These are automagically included in the <head>.
// Note that URLs are relative to your Cargo.toml file.
const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)] // wrap the entire app in a footer
        #[route("/")]
        Index {},

        // This could probably just be a popup.
        // Do we need more than one route?
        #[route("/settings")]
        Settings {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// TODO: #46764e is a great color for text

#[component]
fn Wrapper() -> Element {
    rsx! {
        // nav {
        //     Link { to: Route::Home {}, class: "nav-btn", "Home" }
        //     Link { to: Route::Settings {}, class: "nav-btn", "Settings" }
        // }
        div {
            class: "flex flex-col min-h-screen bg-amber-100 text-green-800", // yellow-50
            // TODO: get header as text+font+svg
            header {
                id: "header",
                class: "mt-4",
                img {
                    class: "object-fill w-[600px] sm:w-[900px] block mx-auto",
                    src: "heading-2.gif"
                }
                // h1 { class: "text-5xl text-center text-emerald-600", "birdtalk" } // cyan-600
            }
            div {
                id: "content",
                Outlet::<Route> {}
            }
            footer {
                id: "footer",
                class: "sticky top-[100vh] grid justify-items-center justify-center",
                div {
                    class: "text-green-800/75",
                    "© 2024 birdtalk"
                }
                // Link { to: Route::Settings {}, class: "nav-btn", "Settings" }
            }
        }
    }
}

#[component]
fn Index() -> Element {
    rsx!(div {
        class: "container m-auto p-4",
        div {
            class: "grid grid-cols-1 justify-items-center place-content-center gap-4",
            div {
                class: "",
                AudioPlayer {}
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                for bird in bird::test_bird_data() {
                    MultipleChoiceCard { bird }
                }
            }
        }
    })
}

// TODO: https://discord.com/channels/899851952891002890/943190605067079712/1178099006811951114
// can use onmounted event to get to html element APIs like `play()`
// then follow a tutorial for a tailwind based player
#[component]
fn AudioPlayer() -> Element {
    rsx! {
        audio {
            controls: "true",
            preload: "auto",
            r#loop: "true",
            // autoplay: "true",
            source {
                r#type: "audio/mpeg",
                src: "/sounds/eurasian_wren.mp3"
            }
            "Your browser does not support the audio element."
        }
    }
}

#[component]
fn MultipleChoiceCard(bird: Bird) -> Element {
    rsx! {
        button {
            class: "group py-8 px-8 w-full mx-auto rounded-xl shadow-lg space-y-2 border border-amber-200 bg-amber-50 hover:bg-amber-200 focus:outline-none focus:ring-2 focus:ring-amber-600 focus:ring-offset-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6",
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird.img_file.to_string_lossy().to_string(),
                alt: bird.common_name,
            }
            div {
                class: "text-center space-y-2 sm:text-left",
                div {
                    class: "space-y-0.5",
                    p {
                        class: "text-lg text-amber-950 font-semibold group-hover:text-green-800",
                        "{bird.common_name}"
                    }
                    p {
                        class: "text-slate-500 font-medium group-hover:text-green-800/75",
                        "{bird.scientific_name}"
                    }
                }
            }
        }
    }
}

#[component]
fn Settings() -> Element {
    rsx!(
        h1 {
            class: "text-2xl text-center",
            "Settings"
        }
        p { "Settings are consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
    )
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
