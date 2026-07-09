use std::cmp::Reverse;
use std::mem;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{bird::Bird, stats::LEARN_THRESHOLD};

pub const MULTIPLE_CHOICE_SIZE: usize = 4;

/// The game data.
#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    /// The current bird choices.
    ///
    /// The first element is the current bird to be identified.
    choices: Vec<BirdContext>,

    /// Birds that have sat out at least 2 full rounds. Kept sorted by weight,
    /// ascending (most overdue first) -- the pool set_next_challenge draws from.
    pack: Vec<BirdContext>,

    /// Birds shown exactly 1 round ago. Hard-excluded from the next round
    /// unless `pack` can't fill one on its own.
    benched: Vec<BirdContext>,

    /// Has this bird pack already been learned?
    already_learned: bool,
}

impl Game {
    /// Initialize the game.
    pub fn init(birds: Vec<Bird>, shuffle: bool) -> Self {
        let mut choices: Vec<_> = birds.into_iter().map(BirdContext::from).collect();
        if shuffle {
            choices.shuffle(&mut thread_rng());
        }
        let pack = choices.split_off(MULTIPLE_CHOICE_SIZE);

        Self {
            choices,
            pack,
            benched: Vec::new(),
            already_learned: false,
        }
    }

    /// Get the current multiple choices.
    pub fn choices(&self) -> &[BirdContext] {
        &self.choices
    }

    /// Get the current multiple choice birds.
    pub fn birds(&self) -> Vec<Bird> {
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

    /// Birds shown last round are always off limits -- a hard 1-round floor is
    /// always achievable. Birds shown 2 rounds ago stay off limits too,
    /// *unless* `pack` can't fill a round on its own: with only 10 birds and 4
    /// shown per round, 8 are always locked up between the current and
    /// just-finished rounds, leaving only 2 truly free, so this kicks in most
    /// rounds. When that happens, `draw_next_choices` reluctantly reuses
    /// whichever benched birds need it *least* (highest weight) so the birds
    /// we're actually trying to give a break aren't the ones getting dragged
    /// back.
    pub fn set_next_challenge(&mut self) {
        let mut previous_choices = mem::take(&mut self.choices);
        let mut next_choices = self.draw_next_choices();

        next_choices.sort_by_key(Self::weight);
        next_choices
            .iter_mut()
            .for_each(|ctx| ctx.last_seen = Some(0));
        previous_choices
            .iter_mut()
            .for_each(|ctx| ctx.last_seen = Some(1));
        // Everyone left in `pack` or `benched` waited one more round for this
        // draw; bump their clock before `benched` graduates into `pack` below.
        self.pack
            .iter_mut()
            .chain(self.benched.iter_mut())
            .for_each(|ctx| {
                if let Some(ls) = ctx.last_seen.as_mut() {
                    *ls += 1;
                }
            });

        self.pack.append(&mut self.benched);
        self.pack.shuffle(&mut thread_rng());
        self.pack.sort_by_key(Self::weight);

        self.benched = previous_choices;
        self.choices = next_choices;
    }

    /// Pop the next round's birds off `self.pack`/`self.benched`, leaving
    /// whatever's left in each for `set_next_challenge` to age and reshuffle.
    fn draw_next_choices(&mut self) -> Vec<BirdContext> {
        if self.pack.len() >= MULTIPLE_CHOICE_SIZE {
            // `pack` is kept sorted by weight (most overdue first), so the
            // front is exactly who should go next.
            self.pack.drain(0..MULTIPLE_CHOICE_SIZE).collect()
        } else {
            let shortfall = MULTIPLE_CHOICE_SIZE - self.pack.len();
            self.benched.sort_by_key(|ctx| Reverse(Self::weight(ctx)));
            let backfill = self.benched.drain(0..shortfall.min(self.benched.len()));
            mem::take(&mut self.pack)
                .into_iter()
                .chain(backfill)
                .collect()
        }
    }

    /// Score a bird for how overdue it is: higher sorts later (shown less
    /// urgently), so ascending sort puts the most-overdue bird first.
    fn weight(ctx: &BirdContext) -> i32 {
        let mut weight: i32 = 0;
        if ctx.learned() {
            weight += 10;
        }
        // Both terms are capped so neither a long mistake streak nor a long
        // absence can dominate the score forever.
        weight -= ctx.mistaken.min(2) as i32;
        weight -= ctx.last_seen.map(|ls| ls.min(5) as i32).unwrap_or(5);
        weight
    }

    /// Get the count of learned birds out of total birds.
    pub fn progress(&self) -> (usize, usize) {
        let total = self.choices.len() + self.pack.len() + self.benched.len();
        let learned = self
            .choices
            .iter()
            .chain(self.pack.iter())
            .chain(self.benched.iter())
            .filter(|bc| bc.learned())
            .count();
        (learned, total)
    }

    pub fn is_complete(&self) -> bool {
        self.choices
            .iter()
            .chain(self.pack.iter())
            .chain(self.benched.iter())
            .all(|bc| bc.learned())
    }
}

/// A bird with surrounding game context. (Per individual game)
// TODO: perhaps separate contextual information away from birds themselves? like a separate hashmap by bird id?
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BirdContext {
    /// The bird.
    pub bird: Bird,
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

impl From<Bird> for BirdContext {
    fn from(bird: Bird) -> Self {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn fake_pack(n: u64) -> Vec<Bird> {
        (0..n)
            .map(|id| Bird {
                id,
                common_name: format!("bird-{id}"),
                scientific_name: format!("sci-{id}"),
                image: String::new(),
                sounds: vec![],
            })
            .collect()
    }

    #[test]
    fn incorrect_answer_resets_streak_but_keeps_mistaken_count() {
        let mut game = Game::init(fake_pack(10), false);

        game.record_choice(true);
        game.record_choice(true);
        assert_eq!(game.correct_choice().consecutively_identified, 2);

        game.record_choice(false);
        assert_eq!(game.correct_choice().consecutively_identified, 0);
        assert_eq!(game.correct_choice().mistaken, 1);
        // `identified` only counts correct answers, so it's untouched by the miss.
        assert_eq!(game.correct_choice().identified, 2);
    }

    #[test]
    fn bird_is_learned_after_learn_threshold_consecutive_corrects() {
        let mut game = Game::init(fake_pack(10), false);

        for _ in 0..LEARN_THRESHOLD {
            assert!(!game.correct_choice().learned());
            game.record_choice(true);
        }

        assert!(game.correct_choice().learned());
    }

    #[test]
    fn is_complete_only_once_every_bird_is_learned() {
        let mut game = Game::init(fake_pack(8), false);
        assert!(!game.is_complete());

        // Each round only advances whichever bird is `correct_choice()`, so
        // keep answering correctly and advancing rounds until every bird has
        // racked up LEARN_THRESHOLD consecutive corrects.
        let mut rounds = 0;
        while !game.is_complete() {
            game.record_choice(true);
            game.set_next_challenge();
            rounds += 1;
            assert!(rounds < 1000, "did not converge -- possible infinite loop");
        }

        let (learned, total) = game.progress();
        assert_eq!(learned, total);
    }

    #[test]
    fn a_bird_never_reappears_within_one_round_of_being_shown() {
        let mut game = Game::init(fake_pack(10), true);
        let mut last_shown_round: HashMap<u64, i32> = HashMap::new();

        for round in 0..200 {
            for ctx in game.choices() {
                if let Some(&prev) = last_shown_round.get(&ctx.bird.id) {
                    assert!(
                        round - prev >= 2,
                        "bird {} reappeared after only {} round(s)",
                        ctx.bird.id,
                        round - prev
                    );
                }
                last_shown_round.insert(ctx.bird.id, round);
            }
            game.record_choice(true);
            game.set_next_challenge();
        }
    }
}
