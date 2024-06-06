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
    stats::Stats,
    ui::{components::Modal, GameStatus, GAME_STATUS},
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
    /// Birdpack ID
    birdpack_id: ReadOnlySignal<String>,
    /// Signal that the game has been completed
    ///
    /// This could be derived from the game signal, but that would have the completed game flow
    /// occur as soon as the user selects the last bird. Perhaps we'll consider that later. For
    /// now, we trigger that flow after the user clicks the "next" button on the flipped card.
    game_completed: Signal<bool>,
}

impl GameCtx {
    fn new(birdpack: BirdPack) -> Self {
        let birdpack_id = use_signal(|| birdpack.id().to_string()).into();
        let game = use_signal(|| Game::init(birdpack, true));
        let stats =
            use_synced_storage::<LocalStorage, _>("{user.email}".to_string(), Stats::default);
        let correct_chosen = use_signal(|| false);
        let game_completed = use_signal(|| false);
        Self {
            game,
            stats,
            correct_chosen,
            birdpack_id,
            game_completed,
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
            stats.add_correct_id(choice.learned(), choice.bird.id());
        } else {
            stats.add_incorrect_id(choice.bird.id());
        }
        self.correct_chosen.set(correct);
    }

    fn next(&mut self) {
        if self.game.read().is_complete() {
            // TODO this could maybe be a case for `use_callback` instead of saving birdpack id?
            self.stats
                .write()
                .add_pack_completed(&self.birdpack_id.read());
            self.game_completed.set(true);
        } else {
            self.correct_chosen.set(false);
            self.game.write().set_next_challenge();
            tracing::debug!(
                "Set new challenge! new bird is: {:?}",
                self.game.read().correct_choice().bird.common_name
            );
        }
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

    let game_over_dismissed_cb = use_callback(move || {
        *GAME_STATUS.write() = GameStatus::None;
    });

    rsx! {
        div {
            class: "container m-auto px-2 landscape:max-lg:px-1 sm:px-4",
            div {
                class: "flex flex-col sm:max-lg:landscape:flex-row justify-center items-center place-content-center gap-2 sm:gap-4",
                ProgressBar {game_ctx}
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
        if *game_ctx.game_completed.read() {
            Modal {
                on_dismiss: game_over_dismissed_cb,
                div {
                    class: "p-2 sm:p-4 mx-auto my-2 flex flex-col items-center gap-4 text-center",
                    h1 {
                        class: "text-2xl",
                        "Nice work!"
                    }
                    table { class: "table-auto text-lg",
                        tbody {
                            tr {
                                td { class: "text-right", "XP:" }
                                td { class: "text-left px-1", "2000", }
                                td { class: "text-left px-1 text-sm text-green-400 font-semibold", "+100"}
                            }
                            tr {
                                td { class: "text-right px-1", "Birds Learned:" }
                                td { class: "text-left px-1", "10", }
                                td { class: "text-left px-1 text-sm text-green-400 font-semibold", "+10"}
                            }
                        }
                    }
                    button {
                        class: "px-4 py-2 focus:outline-none focus-visible:ring focus-visible:ring-green-400 font-semibold text-base bg-green-800 text-amber-50 rounded-full shadow-lg",
                        onclick: move |_| {
                            game_over_dismissed_cb.call()
                        },
                        "Continue"
                    }
                }
            }
        }
    }
}

#[component]
fn ProgressBar(game_ctx: GameCtx) -> Element {
    let progress = game_ctx.percent_complete();
    tracing::debug!("Progress: {}", progress);
    rsx! {
        div {
            class: "h-2 w-3/4 m-4 bg-stone-300/75 rounded-full max-w-screen-md",
            div {
                class: "bg-gradient-to-r from-green-200 to-green-700 min-w-7 h-full rounded-full relative transition-[width,transform]",
                style: "width: min(calc(100% + 0.5rem), calc({progress}% + 1rem))", // 2 rem == w-8
                span {
                    class: "absolute right-0 h-8 w-8 rounded-full object-cover top-[-0.75rem] bg-green-700 bg-[url('/static_logo_transparent.png')] bg-cover",
                }
            }
        }
    }
}
