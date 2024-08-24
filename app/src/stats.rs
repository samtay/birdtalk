//! This module contains progress functionality that is _permanent_ rather than per game session.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};


/// The number of times a bird must be correctly identified consecutively to be considered learned.
pub const LEARN_THRESHOLD: u32 = 3;
pub const BIRDS_PER_LEVEL: usize = 15;

/// Learning progress for a user.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    /// Per bird stats
    bird_stats: HashMap<u64, BirdStats>,

    /// Per pack stats
    pack_stats: HashMap<u64, BirdPackStats>,

    /// Current consecutive correct ID streak.
    current_streak: u32,

    /// Record number of consecutive correct bird IDs.
    record_streak: u32,
}

/// Stats per bird for a user.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirdStats {
    identified: u32,
    mistaken: u32,
    learned: bool,
}

/// Stats per bird pack for a user.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirdPackStats {
    times_completed: usize,
}

impl Stats {
    pub fn xp(&self) -> u32 {
        self.bird_stats
            .values()
            .map(|bs| if bs.learned { 10 } else { 0 } + bs.identified)
            .sum()
    }

    pub fn birds_learned(&self) -> u32 {
        self.bird_stats.values().filter(|bs| bs.learned).count() as u32
    }

    pub fn level(&self) -> u32 {
        1 + (self.bird_stats.values().filter(|bs| bs.learned).count() / BIRDS_PER_LEVEL) as u32
    }

    pub fn add_correct_id(&mut self, learned: bool, bird_id: u64) {
        self.current_streak += 1;
        if self.current_streak > self.record_streak {
            self.record_streak = self.current_streak;
        }
        let bird_stat = self.bird_stats.entry(bird_id).or_default();
        bird_stat.identified += 1;
        bird_stat.learned |= learned;
    }

    pub fn add_incorrect_id(&mut self, bird_id: u64) {
        self.current_streak = 0;
        let bird_stat = self.bird_stats.entry(bird_id).or_default();
        bird_stat.mistaken += 1;
    }

    pub fn add_pack_completed(&mut self, pack_id: u64) {
        let pack_stat = self.pack_stats.entry(pack_id).or_default();
        pack_stat.times_completed += 1;
    }
}
