mod audio;
mod card;
mod modal;
pub mod quiz;

use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use serde::{Deserialize, Serialize};

use super::USE_LOADING_ANIMATION;
use crate::{
    bird::{Bird, BirdPack},
    stats::{Stats, LEARN_THRESHOLD},
};
use audio::AudioPlayer;
use card::MultipleChoiceCard;
use quiz::{Game, MULTIPLE_CHOICE_SIZE};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameMode {
    Listen,
    #[default]
    Learn,
    Quiz,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameMode::Listen => "Listen",
            GameMode::Learn => "Learn",
            GameMode::Quiz => "Quiz",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for GameMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Listen" => Ok(GameMode::Listen),
            "Learn" => Ok(GameMode::Learn),
            "Quiz" => Ok(GameMode::Quiz),
            s => Err(format!("Invalid game mode: {s}")),
        }
    }
}

impl GameMode {
    pub fn description(&self) -> &str {
        match self {
            GameMode::Listen => "Just listen to birds",
            GameMode::Learn => "Listen to birds and then recall them",
            GameMode::Quiz => "Think you know these birds? Test yourself",
        }
    }

    pub fn pressure(&self) -> &str {
        match self {
            GameMode::Listen => "no pressure",
            GameMode::Learn => "a little pressure",
            GameMode::Quiz => "maximum pressure",
        }
    }
}

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
    fn new(birdpack: BirdPack) -> Self {
        let game = use_signal(|| Game::init(birdpack, true));
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
    }

    // TODO: Handle learning the pack! Then visualize progress to learning the pack
    fn next_challenge(&mut self) {
        self.correct_chosen.set(false);
        self.game.write().set_next_challenge();
        tracing::debug!(
            "Set new challenge! new bird is: {:?}",
            self.game.read().correct_choice().bird.common_name
        );
    }
}

#[component]
pub fn GameView(pack: BirdPack, mode: GameMode) -> Element {
    // if USE_LOADING_ANIMATION {
    //     let game = use_signal(|| Game::init_demo(true));
    //     if cfg!(feature = "web") && generation() == 0 {
    //         needs_update();
    //     }
    //     if cfg!(feature = "server") || generation() == 0 {
    //         rsx! {Loading {}}
    //     } else {
    //         rsx! {GameView { game }}
    //     }
    // } else {
    //     let game = use_signal(|| Game::init_demo(false));
    //     rsx! {GameView { game }}
    // }
    if mode != GameMode::Quiz {
        return rsx! { "Not implemented!" };
    }
    let game_ctx = GameCtx::new(pack);
    let shuffle = game_ctx.shuffle_memo();
    let correct_bird = game_ctx.correct_bird_memo();

    rsx! {
        div {
            class: "container m-auto px-2 landscape:max-lg:px-1 sm:px-4",
            div {
                class: "flex flex-col sm:max-lg:landscape:flex-row justify-center items-center place-content-center gap-2 sm:gap-4",
                div {
                    class: "",
                    AudioPlayer {
                        bird: correct_bird
                    }
                }
                div {
                    class: "grid grid-cols-2 gap-4 sm:gap-6 sm:max-lg:landscape:gap-2",
                    for ix in shuffle() {
                        MultipleChoiceCard {
                            bird: game_ctx.game.map(move |g| &g.choices()[ix]),
                            correct: ix == 0,
                            game_ctx
                        }
                    }
                }
            }
        }
    }
}
