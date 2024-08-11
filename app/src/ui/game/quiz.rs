use std::mem;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{
    bird::{BirdDetailed, BirdPackDetailed},
    stats::LEARN_THRESHOLD,
};

pub const MULTIPLE_CHOICE_SIZE: usize = 4;

// TODO move to ui dir?
// Keep persistent / db related stuff like progress outside of UI, put this "flow" stuff into
// UI.

// Game should maybe have some enum field with explains the state -> Next Challenge Ready, Finished Pack, etc.
/// The game data.
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    /// The current bird choices.
    ///
    /// The first element is the current bird to be identified.
    choices: Vec<BirdContext>,

    /// The rest of the birdpack.
    pack: Vec<BirdContext>,

    /// Has this bird pack already been learned?
    already_learned: bool,
}

impl Game {
    /// Initialize the game.
    pub fn init(bird_pack: BirdPackDetailed, shuffle: bool) -> Self {
        let mut choices: Vec<_> = bird_pack.birds.into_iter().map(BirdContext::from).collect();
        if shuffle {
            choices.shuffle(&mut thread_rng());
        }
        let pack = choices.split_off(MULTIPLE_CHOICE_SIZE);

        Self {
            choices,
            pack,
            already_learned: false,
        }
    }

    /// Get the current multiple choices.
    pub fn choices(&self) -> &[BirdContext] {
        &self.choices
    }

    /// Get the current multiple choice birds.
    pub fn birds(&self) -> Vec<BirdDetailed> {
        self.choices.clone().into_iter().map(|bc| bc.bird).collect()
    }

    /// Get the correct choice.
    pub fn correct_choice(&self) -> &BirdContext {
        self.choices.first().unwrap()
    }

    /// Get a mutable reference to the correct choice.
    pub fn correct_choice_mut(&mut self) -> &mut BirdContext {
        self.choices.first_mut().unwrap()
    }

    /// Record a choice made on the current challenge.
    pub fn record_choice(&mut self, correct: bool) {
        let choice = self.correct_choice_mut();
        if correct {
            choice.identified += 1;
            choice.consecutively_identified += 1;
        } else {
            choice.mistaken += 1;
            choice.consecutively_identified = 0;
        }
    }

    // TODO: redo this. it doesn't allow any of the birds in a multi choice round to be repeated?
    // The correct bird choice should be sorted appropriately, but the rest of choices should be
    // either random or chosen with a different criteria.
    // TODO: last seeen should probably only matter for the correct choice?
    // I need to look up if there are standard algorithms for this kind of memorization game...
    pub fn set_next_challenge(&mut self) {
        // Lil over engineered, but mem efficient. self.pack becomes the new choices
        let mut rest_of_pack = self.pack.split_off(MULTIPLE_CHOICE_SIZE);
        mem::swap(&mut self.pack, &mut self.choices);
        self.choices.iter_mut().for_each(|ctx| {
            ctx.last_seen = Some(0);
        });
        self.pack.iter_mut().for_each(|ctx| {
            ctx.last_seen = Some(1);
        });
        rest_of_pack.iter_mut().for_each(|ctx| {
            if let Some(ls) = ctx.last_seen.as_mut() {
                *ls += 1;
            }
        });
        self.pack.append(&mut rest_of_pack);

        // Keep sorted for next time
        self.pack.shuffle(&mut thread_rng());
        self.pack.sort_by_key(|ctx| {
            // The weights might need some randomization too.
            let mut weight: i32 = 0;
            if ctx.learned() {
                weight += 10;
            }
            // TODO: this assumes context is for a given game round. is that what we're doing here?
            weight -= ctx.mistaken as i32;
            weight -= ctx.last_seen.map(|ls| ls.max(5) as i32).unwrap_or(5);
            weight
        });
    }

    pub fn percent_complete(&self) -> usize {
        // TODO: this also assumes an even partition into choices/pack.
        let total = self.choices.len() + self.pack.len();
        let learned = self
            .choices
            .iter()
            .chain(self.pack.iter())
            .filter(|bc| bc.learned())
            .count();
        learned * 100 / total
    }

    pub fn is_complete(&self) -> bool {
        self.choices
            .iter()
            .chain(self.pack.iter())
            .all(|bc| bc.learned())
    }
}

/// A bird with surrounding game context.
// TODO: contextual information should be separated out? now that the rendering is based on the
// bird only and flow writes to context?
// This would make for less complicated coupling between game and storage...
// And game would simply need a read only signal into a Progress Context.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BirdContext {
    /// The bird.
    pub bird: BirdDetailed,
    /// The number of times the bird has been correctly identified.
    pub identified: u32,
    /// The number of times the bird has been correctly identified consecutively.
    ///
    /// A bird is considered learned when this value reaches LEARN_THRESHOLD.
    pub consecutively_identified: u32,
    /// The number of times the bird has been incorrectly identified.
    pub mistaken: u32,
    /// The number of rounds since this bird was last seen.
    pub last_seen: Option<u32>,
}

impl From<BirdDetailed> for BirdContext {
    fn from(bird: BirdDetailed) -> Self {
        Self {
            bird,
            identified: 0,
            consecutively_identified: 0,
            mistaken: 0,
            last_seen: None,
        }
    }
}

impl BirdContext {
    /// Get the bird's learned status within the given game context.
    pub fn learned(&self) -> bool {
        self.consecutively_identified >= LEARN_THRESHOLD
    }
}