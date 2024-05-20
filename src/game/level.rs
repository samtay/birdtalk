/// The number of times a bird must be correctly identified consecutively to be considered learned.
pub const LEARN_THRESHOLD: u32 = 3;

/// A user's score encompasses all of their achievements.
#[derive(Default)]
pub struct Score {
    /// Total birds learned (see [`LEARN_THRESHOLD`]).
    ///
    /// This is a permanent record. If a user makes a mistake after "learning" a bird, this number stays the same.
    // TODO: in reality this probably needs to be a set of bird identifiers. Then we could have indicators or a section where people can see the birds they have/haven't learned.
    birds_learned: u32,

    /// Total times user has correctly IDed a bird.
    birds_identified: u32,

    /// Current consecutive correct ID streak.
    current_streak: u32,

    /// Record number of consecutive correct bird IDs.
    record_streak: u32,
}

impl Score {
    /// XP level is calculated from bird call memorization performance.
    pub fn xp(&self) -> u32 {
        self.birds_learned * 10 + self.birds_identified
    }

    /// Level index starting with 1. This is a simple linear scale, unlike many games that
    /// require more and more XP per level. Also, level depends on _learning_.
    pub fn level(&self) -> u32 {
        self.birds_learned / 10
    }

    pub fn add_correct_id(&mut self) {
        self.birds_identified += 1;
        self.current_streak += 1;
        if self.current_streak > self.record_streak {
            self.record_streak = self.current_streak;
        }
    }

    pub fn add_incorrect_id(&mut self) {
        self.current_streak = 0;
    }
}
