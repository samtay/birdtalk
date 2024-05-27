mod audio;
mod card;
mod modal;

use dioxus::prelude::*;

use crate::{
    game::{Game, MULTIPLE_CHOICE_SIZE},
    ui::USE_LOADING_ANIMATION,
};
use audio::AudioPlayer;
use card::MultipleChoiceCard;

#[component]
pub fn GameView(game: Signal<Game>) -> Element {
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
                            game,
                            correct_chosen,
                        }
                    }
                }
            }
        }
    }
}
