mod game;

use dioxus::prelude::*;

use crate::game::Game;
use game::GameView;

const USE_LOADING_ANIMATION: bool = false;

const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)] // wrap the entire app between header/footer
        #[route("/")]
        Index {},

        // This could probably just be a popup.
        // Do we need more than one route?
        #[route("/settings")]
        Settings {},
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-dvh bg-amber-100 text-green-800",
            header {
                id: "header",
                class: "shrink container h-32 sm:h-48 md:h-64 w-full max-w-screen-lg mt-2 sm:mt-4 mb-[-2rem] mx-auto sm:max-md:landscape:hidden bg-[url('heading-2.gif')] bg-cover bg-center bg-no-repeat",
            }
            div {
                id: "content",
                class: "no-shrink",
                Outlet::<Route> {}
            }
            footer {
                id: "footer",
                class: "shrink sticky top-[100vh] grid justify-items-center justify-center sm:max-md:landscape:hidden",
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
    // Trigger an initial re-render on the frontend to load the game.
    //
    // This is necessary because the randomness causes the SSR and first frontend render to be out
    // of sync.
    //
    // A further optimization: Have a "GameView" placeholder with the loading symbol where the
    // audio button goes (as it is now) plus grayed out content cards where the birds are.
    //
    // Note: I don't think any of this will be necessary in a future state. The initial page should
    // require auth on the backend to choose available bird packs _and_ requesting local storage to
    // see what's immediately available and what needs downloading. Randomization will always be
    // client side. For now, just use a loading symbol to keep random game init on the client side
    // on page load.
    //
    // Yet another way around this: use_server_future(|| rand::<SeedType>()) have the server
    // generate a random seed for the initial game.
    if USE_LOADING_ANIMATION {
        let game = use_signal(|| Game::init_demo(true));
        if cfg!(feature = "web") && generation() == 0 {
            needs_update();
        }
        if cfg!(feature = "server") || generation() == 0 {
            rsx! {Loading {}}
        } else {
            rsx! {GameView { game }}
        }
    } else {
        let game = use_signal(|| Game::init_demo(false));
        rsx! {GameView { game }}
    }
}

#[component]
fn Loading() -> Element {
    rsx! {
        div {
            class: "mt-12 flex flex-col items-center justify-center", // animate-bounce
            div {
                class: "animate-spin w-24 h-24 border-t-4 border-green-800 rounded-full",
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
