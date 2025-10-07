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

/// Represents the semantic "valence" of a concept.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Valence {
    Negative,
    Positive,
    Neutral,
    Uncertain,
}

/// The context required to make a placement decision for a new concept.
pub struct PlacementContext<'a> {
    /// The proposed representation vector for the new concept.
    pub vector: &'a [f64],
    /// The result of the Principle of Compositionality check.
    pub composition_ok: bool,
    /// The result of the Principle of Symmetry check.
    pub symmetry_ok: bool,
}

/// Represents the outcome of the placement logic, including the final valence
/// and the reason for the decision.
#[derive(Debug, PartialEq, Eq)]
pub enum PlacementDecision {
    /// The concept is placed with high confidence.
    Placed(Valence),
    /// The concept is placed in the Neutral Zone due to a failed check.
    ProjectedToNeutral,
    /// The concept is placed in the Zone of Uncertainty as a last resort.
    FellToUncertainty,
}

/// Determines the final placement of a new concept based on the core principles.
/// This is the heart of the "Big Compute" Decomposer's logic.
pub fn determine_placement(context: &PlacementContext) -> PlacementDecision {
    if !context.composition_ok {
        // A failed compositionality check is a critical anomaly.
        // The concept cannot be placed; it must be flagged for resolution.
        // For now, we treat it as uncertain.
        return PlacementDecision::FellToUncertainty;
    }

    let valence_value = context.vector.first().cloned().unwrap_or(0.0);
    let initial_valence = get_valence(valence_value);

    if context.symmetry_ok {
        // Both composition and symmetry are OK. We can place the concept with high confidence.
        PlacementDecision::Placed(initial_valence)
    } else {
        // Composition is OK, but symmetry is not. The concept is "unrefined."
        // We cannot confidently place it in a Positive or Negative zone.
        match initial_valence {
            Valence::Positive | Valence::Negative => {
                // Project the unrefined concept to the Neutral Zone.
                PlacementDecision::ProjectedToNeutral
            }
            // If it was already Neutral or Uncertain, the lack of symmetry doesn't change that.
            Valence::Neutral => PlacementDecision::Placed(Valence::Neutral),
            Valence::Uncertain => PlacementDecision::Placed(Valence::Uncertain),
        }
    }
}

/// Determines the Valence of a concept from the first dimension of its vector.
pub fn get_valence(value: f64) -> Valence {
    if value < UNCERTAINTY_NEG_BOUND {
        Valence::Negative
    } else if value > UNCERTAINTY_POS_BOUND {
        Valence::Positive
    } else if value > UNCERTAINTY_NEG_BOUND && value < UNCERTAINTY_POS_BOUND {
        Valence::Uncertain
    } else {
        Valence::Neutral
    }
}
