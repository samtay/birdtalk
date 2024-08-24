mod audio;
mod card;
mod game_over;
pub mod quiz;

use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{
    bird::{Bird, BirdPack},
    stats::Stats,
    ui::AppCtx,
};
use audio::AudioPlayer;
use card::MultipleChoiceCard;
use game_over::GameOverModal;
use quiz::{Game, MULTIPLE_CHOICE_SIZE};

#[derive(Clone, Copy)]
struct GameCtx {
    /// Game state
    game: Signal<Game>,
    /// Storage backed stats state
    stats: Signal<Stats>,
    /// Value of `stats` at the game start (so we can diff at the end).
    stats_original: ReadOnlySignal<Stats>,
    /// Has a correct choice been made for this multiple choice yet?
    correct_chosen: Signal<bool>,
    /// Birdpack ID
    birdpack_id: ReadOnlySignal<u64>,
    /// Signal that the game has been completed
    ///
    /// This could be derived from the game signal, but that would have the completed game flow
    /// occur as soon as the user selects the last bird. Perhaps we'll consider that later. For
    /// now, we trigger that flow after the user clicks the "next" button on the flipped card.
    game_completed: Signal<bool>,
    /// For convenience
    app_ctx: AppCtx,
}

impl GameCtx {
    /// Initialize a new game context (and provide it to children).
    fn init(birdpack: BirdPack) -> Self {
        let app_ctx = use_context::<AppCtx>();
        let birdpack_id = use_signal(|| birdpack.id).into();
        let game = use_signal(|| Game::init(birdpack, true));
        let stats = *app_ctx.stats;
        let stats_original = stats.with_peek(|og| {
            tracing::debug!("Generation: {}, Original stats: {:?}", generation(), og);
            Signal::new(og.clone()).into()
        });
        let correct_chosen = use_signal(|| false);
        let game_completed = use_signal(|| false);
        use_context_provider(|| Self {
            game,
            stats,
            correct_chosen,
            birdpack_id,
            game_completed,
            stats_original,
            app_ctx,
        })
    }

    /// Create a new memo signal of the current challenge's birds
    fn birds_memo(&self) -> Memo<Vec<Bird>> {
        let game = self.game;
        use_memo(move || game.read().birds())
    }

    /// Create a new memo signal of the current correct bird
    fn correct_bird_memo(&self) -> Memo<Bird> {
        let game = self.game;
        use_memo(move || game.read().correct_choice().bird.clone())
    }

    /// Create a shuffle of 0..4 that will shuffle itself on changes to birds
    // Can maybe subscribe to a "turn" so shuffle only runs per turn
    // For now just hack this to change when the actual birds change
    fn shuffle_memo(&self) -> Memo<Vec<usize>> {
        let birds = self.birds_memo();
        use_memo(move || {
            let _ = birds.read(); // subscribe to birds
            let mut indices = (0..MULTIPLE_CHOICE_SIZE).collect::<Vec<_>>();
            indices.shuffle(&mut rand::thread_rng());
            indices
        })
    }

    fn percent_complete(&self) -> Memo<usize> {
        let game = self.game;
        use_memo(move || game.read().percent_complete())
    }

    fn record_choice(&mut self, correct: bool) {
        self.game.write().record_choice(correct);
        let game = self.game.read();
        let choice = game.correct_choice();
        let mut stats = self.stats.write();
        if correct {
            stats.add_correct_id(choice.learned(), choice.bird.id);
        } else {
            stats.add_incorrect_id(choice.bird.id);
        }
        self.correct_chosen.set(correct);
    }

    async fn next(&mut self) {
        if self.game.read().is_complete() {
            self.stats
                .write()
                .add_pack_completed(*self.birdpack_id.read());
            self.game_completed.set(true);
            if let Err(e) = self.app_ctx.stats.sync().await {
                // TODO: Display error to user
                tracing::error!("Failed to sync stats: {:?}", e);
            }
        } else {
            // Cards flip back to face up
            self.correct_chosen.set(false);

            // Try to let that flip complete before the new birds are displayed
            #[cfg(feature = "web")]
            async_std::task::sleep(std::time::Duration::from_millis(300)).await;

            // Continue with next challenge
            self.game.write().set_next_challenge();
            tracing::debug!(
                "Set new challenge! new bird is: {:?}",
                self.game.read().correct_choice().bird.common_name
            );
        }
    }
}

#[component]
pub fn GameView(pack: BirdPack) -> Element {
    let game_ctx = GameCtx::init(pack);
    let shuffle = game_ctx.shuffle_memo();
    let correct_bird = game_ctx.correct_bird_memo();

    rsx! {
        div {
            class: "container m-auto px-2 sm:px-4",
            "inert": game_ctx.game_completed.read().then(|| true),
            div {
                class: "flex flex-col justify-center items-center place-content-center gap-2 sm:gap-4",
                ProgressBar { }
                div {
                    class: "",
                    AudioPlayer {
                        bird: correct_bird
                    }
                }
                div {
                    class: "grid grid-cols-2 gap-4 sm:gap-6",
                    for ix in shuffle() {
                        MultipleChoiceCard {
                            bird: game_ctx.game.map(move |g| &g.choices()[ix]),
                            correct: ix == 0,
                        }
                    }
                }
            }
        }
        if *game_ctx.game_completed.read() {
            GameOverModal { }
        }
    }
}

// TODO:
#[component]
pub fn GameViewPlaceholder() -> Element {
    rsx! {
        div { " nice place holder ! "}
    }
}

#[component]
fn ProgressBar() -> Element {
    let game_ctx = use_context::<GameCtx>();
    let progress = game_ctx.percent_complete();
    tracing::debug!("Progress: {}", progress);
    const LOGO: &str = manganis::mg!("assets/static_logo_transparent.png");
    rsx! {
        div {
            class: "h-2 w-3/4 m-4 mt-6 bg-offwhite-2 rounded-full max-w-screen-md",
            div {
                class: "bg-gradient-to-r from-green-light to-green-dark min-w-7 h-full rounded-full relative transition-[width,transform]",
                style: "width: min(calc(100% + 0.5rem), calc({progress}% + 1rem))", // 2 rem == w-8
                span {
                    class: "absolute right-0 h-8 w-8 rounded-full object-cover top-[-0.75rem] bg-green-dark bg-cover",
                    background_image: "url({LOGO})"
                }
            }
        }
    }
}
