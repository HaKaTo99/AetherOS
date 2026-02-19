//! Intent Recognition Model (TinyML Pattern Recognition)
//!
//! Provides high-level pattern matching for user behavior audit.

use crate::ai::intent::UserIntent;

pub struct IntentModel {
    // Military Grade: Deterministic Scoring System
    dev_weight: u32,
    sec_weight: u32,
    media_weight: u32,
}

impl IntentModel {
    pub const fn new() -> Self {
        Self {
            dev_weight: 1,
            sec_weight: 5,
            media_weight: 3,
        }
    }

    /// Predicts intent based on a score map
    pub fn predict(&self, dev_score: u32, sec_score: u32, media_score: u32) -> UserIntent {
        let scores = [
            (UserIntent::Development, dev_score * self.dev_weight),
            (UserIntent::SecurityAudit, sec_score * self.sec_weight),
            (UserIntent::Multimedia, media_score * self.media_weight),
        ];

        let mut best_intent = UserIntent::General;
        let mut max_score = 15; // Stability Threshold

        for (intent, score) in scores.iter() {
            if *score > max_score {
                max_score = *score;
                best_intent = *intent;
            }
        }

        best_intent
    }
}
