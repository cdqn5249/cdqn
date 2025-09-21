//! CDQN v3.0 - Pure Functional Sovereign Core
//!
//! From absolute zero, the causal universe expands through pure transformations.

pub mod kdu;
pub mod world;
pub mod entities;
pub mod modules;
pub mod protocol;
pub mod economics;

pub use kdu::CuriosityKdu;
pub use world::{ImmutableWorld, WorldTransformation};

/// Pure genesis - the empty causal universe
pub fn genesis_world(fqei: &str) -> ImmutableWorld {
    ImmutableWorld::genesis(fqei)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_zero() {
        let world = genesis_world("genesis:node");
        assert_eq!(world.hlc_state.logical_counter, 0);
        assert_eq!(world.journal.len(), 0);
        // The universe begins empty, pure, ready for causal evolution
    }
}
