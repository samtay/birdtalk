#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;
use tracing::Level;

use crate::{
    bird::Bird,
    game::{Game, MULTIPLE_CHOICE_SIZE},
};

mod bird;
mod game;

const USE_LOADING_ANIMATION: bool = false;

// These are automagically included in the <head>.
// Note that URLs are relative to your Cargo.toml file.
// const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

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
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App)
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
        div {
            class: "flex flex-col min-h-screen bg-amber-100 text-green-800", // yellow-50
            header {
                id: "header",
                class: "h-32 sm:h-48 md:h-64 w-full max-w-screen-lg mt-2 sm:mt-4 my-[-1rem] sm:my-[-2rem] mx-auto bg-[url('heading-2.gif')] bg-cover bg-center bg-no-repeat",
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
fn GameView(game: Signal<Game>) -> Element {
    let birds = use_memo(move || {
        let birds = game
            .read()
            .choices()
            .clone()
            .into_iter()
            .map(|bc| bc.bird)
            .collect::<Vec<_>>();
        tracing::debug!(
            "Birds: {:?}",
            birds.iter().map(|b| &b.common_name).collect::<Vec<_>>()
        );
        birds
    });

    // Can maybe subscribe to a "turn" so shuffle only runs per turn
    // For now just hack this to change when the actual birds change
    let shuffle = use_memo(move || {
        let _ = birds.read(); // subscribe to birds
        let mut indices = (0..MULTIPLE_CHOICE_SIZE).collect::<Vec<_>>();
        if USE_LOADING_ANIMATION || (generation() > 0 && cfg!(feature = "web")) {
            use rand::seq::SliceRandom as _;
            indices.shuffle(&mut rand::thread_rng());
            tracing::debug!("Shuffled: {:?}", indices);
        }
        indices
    });

    rsx!(div {
        class: "container m-auto p-2 sm:p-4",
        div {
            class: "grid grid-cols-1 justify-items-center place-content-center gap-2 sm:gap-4",
            div {
                class: "",
                AudioPlayer {
                    bird: birds.map(|bs| {
                        let bird = bs.first().unwrap();
                        tracing::debug!("Bird: {:?}", bird.common_name);
                        bird
                    })
                }
            }
            div {
                class: "grid grid-cols-2 gap-4",
                for ix in shuffle() {
                    MultipleChoiceCard {
                        bird: birds.map(move |bs| &bs[ix]),
                        correct: ix == 0,
                        onclick: move |_| {
                            tracing::debug!("Clicked on choice {}", ix);
                            handle_choice(ix == 0, game)
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn AudioPlayer(bird: MappedSignal<Bird>) -> Element {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlAudioElement;

    let mut audio_element: Signal<Option<HtmlAudioElement>> = use_signal(|| None);
    let mut playing: Signal<bool> = use_signal(|| false);

    // Explicitly audio.load() on changes to bird, otherwise the first audio element gets persisted
    // indefinitely.
    let bird_signal = bird.clone();
    use_effect(move || {
        let _ = bird_signal.read();
        if let Some(audio) = audio_element.read().as_ref() {
            audio.load();
        }
    });

    rsx! {
        button {
            onclick: move |_| async move {
                if let Some(audio) = audio_element.read().as_ref() {
                    tracing::debug!("audio_element.src(): {:?}", audio.current_src());
                    // TODO: determine which of these errors should be displayed.
                    if audio.paused() {
                        if let Ok(promise) = audio.play() {
                            wasm_bindgen_futures::JsFuture::from(promise).await.ok();
                        }
                    } else {
                        audio.pause().ok();
                    }
                }
            },
            svg {
                "viewBox": "0 0 24 24",
                "fill": "none",
                "stroke-width": "1.5",
                "stroke": "currentColor",
                "xmlns": "http://www.w3.org/2000/svg",
                class: "w-16 h-16 sm:w-24 sm:h-24",
                path {
                    "stroke-linejoin": "round",
                    "stroke-linecap": "round",
                    "d": if playing() {
                        "M14.25 9v6m-4.5 0V9M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                    } else {
                        "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z
                         M15.91 11.672a.375.375 0 0 1 0 .656l-5.603 3.113a.375.375 0 0 1-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112Z"
                    }
                }
            }


        }
        audio {
            onmounted: move |cx| audio_element.set(cx.downcast::<web_sys::Element>().cloned().map(|el| el.unchecked_into())),
            onplay: move |_| *playing.write() = true,
            onpause: move |_| *playing.write() = false,
            // controls: "true",
            preload: "auto",
            r#loop: "true",
            autoplay: "true",
            source {
                r#type: "audio/mpeg",
                src: bird.read().sound_file.to_string_lossy().to_string(),
            }
            "Your browser does not support the audio element."
        }
    }
}

// TODO: Probably will need a custom component to handle effects, animations, etc.
#[component]
fn MultipleChoiceCard(
    bird: MappedSignal<Bird>,
    correct: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let bird = bird.read();
    let mut button_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    rsx! {
        button {
            onclick: move |e| async move {
                onclick.call(e);
                // For now, just blur the element so the new one doesn't stay focused.
                // Might be preferable to set focus onto the audio button instead?
                // Would need to mark that one as focus-visible
                if correct {
                    if let Some(el) = button_element.read().as_ref() {
                        el.set_focus(false).await.ok();
                    }
                }
            },
            onmounted: move |e| button_element.set(Some(e.data())),
            class: "group p-4 w-full mx-auto rounded-xl shadow-lg space-y-2 border border-amber-200 bg-amber-50 hover:bg-amber-200 focus:outline-none focus:ring-2 focus:ring-amber-600 focus:ring-offset-2 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6",
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: "{bird.img_file.to_string_lossy()}",
                alt: "{bird.common_name}",
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
                        class: "text-sm sm:text-base text-slate-500 font-medium group-hover:text-green-800/75",
                        "{bird.scientific_name}"
                    }
                }
            }
        }
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

// #[server(PostServerData)]
// async fn post_server_data(data: String) -> Result<(), ServerFnError> {
//     tracing::info!("Server received: {}", data);
//     Ok(())
// }
//
#[server]
async fn get_initial_game() -> Result<Game, ServerFnError> {
    Ok(Game::init_demo(true))
}

// TODO: most of this should live on the game itself
fn handle_choice(correct: bool, game: Signal<Game>) {
    tracing::info!("handle_choice was called");
    let mut game = game;
    let mut game = game.write();
    let choice = game.correct_choice_mut();
    if correct {
        choice.identified += 1;
        choice.consecutively_identified += 1;
        tracing::info!("setting next challenge...");
        game.set_next_challenge();
        tracing::info!(
            "set! new bird is: {:?}",
            game.correct_choice().bird.common_name
        );
    } else {
        choice.mistaken += 1;
        choice.consecutively_identified = 0;
    }
}
