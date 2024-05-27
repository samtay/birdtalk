#![allow(non_snake_case)]

use std::rc::Rc;

use dioxus::prelude::*;
use tracing::Level;

use crate::{
    bird::Bird,
    game::{BirdContext, Game, MULTIPLE_CHOICE_SIZE},
};

mod bird;
mod game;

const USE_LOADING_ANIMATION: bool = false;
const AUDIO_LOOP: bool = true;
const AUDIO_AUTOPLAY: bool = true;

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
    #[allow(clippy::let_unit_value)]
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 3000)))
    );
    LaunchBuilder::fullstack().with_cfg(cfg).launch(App)
    // launch(App)
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-dvh bg-amber-100 text-green-800", // yellow-50
            header {
                id: "header",
                class: "shrink container h-32 sm:h-48 md:h-64 w-full max-w-screen-lg mt-2 sm:mt-4 mb-[-2rem] mx-auto bg-[url('heading-2.gif')] bg-cover bg-center bg-no-repeat",
            }
            div {
                id: "content",
                class: "no-shrink",
                Outlet::<Route> {}
            }
            footer {
                id: "footer",
                class: "shrink sticky top-[100vh] grid justify-items-center justify-center",
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
    let birds = use_memo(move || game.read().birds());

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

    let correct_chosen = use_signal(|| false);

    rsx! {
        div {
            class: "container m-auto px-2 sm:px-4",
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
                    class: "grid grid-cols-2 gap-4 sm:gap-6",
                    for ix in shuffle() {
                        MultipleChoiceCard {
                            bird: game.map(move |g| &g.choices()[ix]),
                            correct: ix == 0,
                            game,
                            correct_chosen,
                        }
                    }
                }
            }
        }
    }
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
            r#loop: AUDIO_LOOP,
            autoplay: AUDIO_AUTOPLAY,
            source {
                r#type: "audio/mpeg",
                src: bird.read().sound_file.to_string_lossy().to_string(),
            }
            "Your browser does not support the audio element."
        }
    }
}

#[component]
fn MultipleChoiceCard(
    bird: MappedSignal<BirdContext>,
    correct: bool,
    game: Signal<Game>,
    correct_chosen: Signal<bool>,
) -> Element {
    let bird_copy = bird.clone();
    let bird_memo = use_memo(move || bird_copy.read().bird.clone());
    let next_button_enabled = use_memo(move || *correct_chosen.read() && correct);
    rsx! {
        div {
            // TODO: try removing this with the other cubic, it might be better fitting vibe.
            class: "[perspective:1000px]",
            div {
                class: "grid transition-transform duration-1000 [transform-style:preserve-3d] h-full",
                class: if correct && correct_chosen() {
                    "[transform:rotateY(180deg)]"
                },
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(0deg)]",
                    CardFront {
                        bird: bird_memo,
                        onclick: move |_| {
                            if correct {
                                correct_chosen.set(true);
                            }
                            modify_choice_stats(correct, game);
                        },
                        correct,
                        correct_chosen,
                    }
                }
                div {
                    class: "row-start-1 row-end-2 col-start-1 col-end-2 [backface-visibility:hidden] [transform:rotateY(-180deg)]",
                    CardBack {
                        bird,
                        onclick: move |_| {
                            if correct {
                                correct_chosen.set(false);
                                next_challenge(game);
                            } else {
                                tracing::error!("This shouldn't happen. How did you get here?");
                            }
                        },
                        next_button_enabled
                    }
                }
            }
        }
    }
}

#[component]
fn CardFront(
    bird: Memo<Bird>,
    correct: bool,
    onclick: EventHandler<MouseEvent>,
    correct_chosen: Signal<bool>,
) -> Element {
    // TODO: note that this is assuming a different set of birds each round!
    let mut mistakenly_chosen = use_signal(|| false);
    use_effect(move || {
        bird.read();
        mistakenly_chosen.set(false);
    });
    rsx! {
        button {
            onclick: move |e| async move {
                // Handle mistaken state
                if !correct {
                    tracing::debug!("Setting mistakenly_chosen to true");
                    mistakenly_chosen.set(true);
                }
                // Let the parent know the choice was made
                onclick.call(e);
            },
            class: "group p-4 w-full h-full mx-auto border-amber-200 rounded-xl shadow enabled:hover:shadow-lg enabled:hover:bg-amber-200 space-y-2 bg-amber-50 focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-600 focus-visible:ring-offset-2 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 disabled:shadow-none",
            class: if mistakenly_chosen() {
                "animate-shake"
            },
            class: if mistakenly_chosen() || correct_chosen() {
                "disabled border opacity-50 transition-opacity duration-1000"
            } else {
                "border-2"
            },
            disabled: mistakenly_chosen() || correct_chosen(),
            img {
                class: "block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().img_file.to_string_lossy().to_string(),
                alt: bird().common_name,
            }
            div {
                class: "text-center space-y-2 sm:text-left",
                div {
                    class: "space-y-0.5",
                    p {
                        class: "text-lg text-amber-950 font-semibold group-enabled:group-hover:text-green-800",
                        "{bird().common_name}"
                    }
                    p {
                        class: "text-sm sm:text-base text-slate-500 font-medium group-enabled:group-hover:text-green-800/75",
                        "{bird().scientific_name}"
                    }
                }
            }
        }
    }
}

#[component]
fn CardBack(
    bird: MappedSignal<BirdContext>,
    onclick: EventHandler<MouseEvent>,
    next_button_enabled: Memo<bool>,
) -> Element {
    let mut next_button: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if next_button_enabled() {
            tracing::debug!("Spawning to focus next button...");
            spawn(async move {
                tracing::debug!("Trying to act on button...");
                if let Some(btn) = next_button.read().as_ref() {
                    tracing::debug!("Setting focus on next button");
                    btn.set_focus(true).await.ok();
                }
            });
        }
    });
    rsx! {
        div {
            class: "p-4 w-full h-full mx-auto border-green-200 rounded-xl shadow space-y-2 bg-green-100/50 sm:px-8 sm:flex sm:items-center sm:space-y-0 sm:space-x-6 border-2",
            img {
                class: "animate-[spin_1s_linear] block mx-auto w-24 h-24 rounded-full object-cover sm:mx-0 sm:shrink-0",
                src: bird().bird.img_file.to_string_lossy().to_string(),
                alt: bird().bird.common_name,
            }
            div {
                class: "text-center sm:text-left w-full",
                div {
                    class: "flex flex-col sm:flex-row justify-between",
                    div {
                        class: "text-lg font-semibold text-green-800 whitespace-nowrap",
                        "Nice work!"
                    }
                    div {
                        class: "flex space-x-2 text-sm text-green-800/75 whitespace-nowrap",
                        div {
                            "Identified: {bird().identified}"
                        }
                        div {
                            "Streak: {bird().consecutively_identified}"
                        }
                    }
                    button {
                        class: "mt-2 px-4 py-2 focus:outline-none focus-visible:ring-2 focus-visible:ring-amber-600 focus-visible:ring-offset-2 font-semibold text-sm sm:text-base bg-green-500 text-amber-50 rounded-full shadow-sm",
                        onclick: move |e| async move {
                            onclick.call(e);
                        },
                        onmounted: move |e| async move {
                            next_button.set(Some(e.data()));
                        },
                        disabled: !next_button_enabled(),
                        "Ok!"
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
fn modify_choice_stats(correct: bool, game: Signal<Game>) {
    tracing::info!("handle_choice was called");
    let mut game = game;
    let mut game = game.write();
    let choice = game.correct_choice_mut();
    if correct {
        choice.identified += 1;
        choice.consecutively_identified += 1;
    } else {
        choice.mistaken += 1;
        choice.consecutively_identified = 0;
    }
}

fn next_challenge(game: Signal<Game>) {
    tracing::info!("setting next challenge...");
    let mut game = game;
    game.write().set_next_challenge();
    tracing::info!(
        "set! new bird is: {:?}",
        game.read().correct_choice().bird.common_name
    );
}
