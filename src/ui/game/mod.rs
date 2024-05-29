mod audio;
mod card;
mod modal;
pub mod quiz;

use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

use super::USE_LOADING_ANIMATION;
use crate::{
    bird::Bird,
    stats::{Stats, LEARN_THRESHOLD},
};
use audio::AudioPlayer;
use card::MultipleChoiceCard;
use quiz::{Game, MULTIPLE_CHOICE_SIZE};

#[derive(Clone, Copy, PartialEq)]
struct GameCtx {
    /// Game state
    game: Signal<Game>,
    /// Storage backed stats state
    stats: Signal<Stats>,
    /// Has a correct choice been made for this multiple choice yet?
    correct_chosen: Signal<bool>,
}

impl GameCtx {
    // TODO right now game signal created with loading nonsense from above.
    // After we split it up, just create it here (with use_hook)
    // ALL of this should be under one use_hook (after we figure out storage panic)
    // probably _outside_ of this so that we could also choose to do context
    fn new(game: Signal<Game>) -> Self {
        let stats =
            use_synced_storage::<LocalStorage, _>("{user.email}".to_string(), Stats::default);
        let correct_chosen = use_signal(|| false);
        Self {
            game,
            stats,
            correct_chosen,
        }
    }

    /// Create a new memo signal of the current challenge's birds
    fn birds_memo(&self) -> Memo<Vec<Bird>> {
        let game = self.game;
        use_memo(move || game.read().birds())
    }

    /// Create a shuffle of 0..4 that will shuffle itself on changes to birds
    // Can maybe subscribe to a "turn" so shuffle only runs per turn
    // For now just hack this to change when the actual birds change
    fn shuffle_memo(&self) -> Memo<Vec<usize>> {
        let birds = self.birds_memo();
        use_memo(move || {
            let _ = birds.read(); // subscribe to birds
            let mut indices = (0..MULTIPLE_CHOICE_SIZE).collect::<Vec<_>>();
            if USE_LOADING_ANIMATION || (generation() > 0 && cfg!(feature = "web")) {
                use rand::seq::SliceRandom as _;
                indices.shuffle(&mut rand::thread_rng());
                tracing::debug!("Shuffled: {:?}", indices);
            }
            indices
        })
    }

    fn record_choice(&mut self, correct: bool) {
        self.game.write().record_choice(correct);
        let game = self.game.read();
        let choice = game.correct_choice();
        let mut stats = self.stats.write();
        if correct {
            stats.add_correct_id(
                choice.consecutively_identified >= LEARN_THRESHOLD,
                choice.bird.id(),
            );
        } else {
            stats.add_incorrect_id(choice.bird.id());
        }
        self.correct_chosen.set(correct);
        // TODO: remove
        drop(stats);
        tracing::debug!("Stats: {:?}", self.stats.read());
    }

    fn next_challenge(&mut self) {
        self.correct_chosen.set(false);
        self.game.write().set_next_challenge();
        tracing::debug!(
            "set! new bird is: {:?}",
            self.game.read().correct_choice().bird.common_name
        );
    }
}

#[component]
pub fn GameView(game: Signal<Game>) -> Element {
    let game_ctx = GameCtx::new(game);
    let birds = game_ctx.birds_memo();
    let shuffle = game_ctx.shuffle_memo();

    rsx! {
        div {
            class: "container m-auto px-2 sm:px-4",
            div {
                class: "grid grid-cols-1 justify-items-center place-content-center gap-2 sm:gap-4 sm:max-md:landscape:gap-2",
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
                    class: "grid grid-cols-2 gap-4 sm:gap-6 sm:max-md:landscape:gap-2",
                    for ix in shuffle() {
                        MultipleChoiceCard {
                            bird: game.map(move |g| &g.choices()[ix]),
                            correct: ix == 0,
                            game_ctx,
                        }
                    }
                }
            }
        }
    }
}
