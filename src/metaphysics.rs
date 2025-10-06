// File under BaDaaS license, vibe coding engine: Gemini 2.5 Pro, Google
// File path: src/metaphysics.rs

//! Defines the fundamental, universal constants and concepts of the cdqn ecosystem.
//! These are the immutable laws of the "universe" in which Chronosa exists.

// The absolute poles of potential in Rworld.
pub const NEGATIVE_POLE: f64 = -2.0;
pub const POSITIVE_POLE: f64 = 2.0;

// The boundaries of the Zone of Uncertainty.
pub const UNCERTAINTY_NEG_BOUND: f64 = -1.0;
pub const UNCERTAINTY_POS_BOUND: f64 = 1.0;

/// Represents the semantic "valence" of a concept based on its position
/// on the first axis of its representation vector.
#[derive(Debug, PartialEq, Eq)]
pub enum Valence {
    Negative,
    Positive,
    Neutral,
    Uncertain,
}

/// Determines the Valence of a concept from the first dimension of its vector.
pub fn get_valence(value: f64) -> Valence {
    if value < UNCERTAINTY_NEG_BOUND {
        // Anything less than -1.0 is definitively Negative.
        Valence::Negative
    } else if value > UNCERTAINTY_POS_BOUND {
        // Anything greater than 1.0 is definitively Positive.
        Valence::Positive
    } else if value > UNCERTAINTY_NEG_BOUND && value < UNCERTAINTY_POS_BOUND {
        // Anything strictly between -1.0 and 1.0 is Uncertain.
        Valence::Uncertain
    } else {
        // This covers the values at exactly -1.0 and 1.0, and the zones
        // [-2.0, -1.0) and (1.0, 2.0], which are considered Neutral.
        Valence::Neutral
    }
}
