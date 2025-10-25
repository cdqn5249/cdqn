// BaDaaS License
// File Path: crates/cdqn_core/src/algebra/mod.rs
//
// Minimal, self-contained implementation of the Prime-Element
// Algebra scaffolding required by cdqn_core.  The goal is to let
// the crate compile successfully while we flesh out full algebraic
// semantics later.
//
// Pure functions and immutability are enforced: the types are all
// `Copy` or `Clone`, and there is no interior mutability.

#![forbid(unsafe_code)]

// ──────────────────────────────────────────────────────────────
// Prime Elements
// ──────────────────────────────────────────────────────────────
pub mod prime {
    use core::fmt;

    /// Charge represents the dual nature (+/-) of every prime element.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Charge {
        /// Positive polarity (e.g., +Growth, +Order)
        Pos,
        /// Negative polarity (e.g., -Decay, -Chaos)
        Neg,
    }

    impl fmt::Display for Charge {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Charge::Pos => write!(f, "+"),
                Charge::Neg => write!(f, "-"),
            }
        }
    }

    /// A **PrimeElement** is an indivisible semantic atom used by the Causal
    /// Engine.  For now we define just a handful; the full vocabulary will grow
    /// organically.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PrimeElement {
        Growth,
        Decay,
        Order,
        Chaos,
    }

    impl fmt::Display for PrimeElement {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use PrimeElement::*;
            let s = match self {
                Growth => "Growth",
                Decay  => "Decay",
                Order  => "Order",
                Chaos  => "Chaos",
            };
            write!(f, "{s}")
        }
    }

    // Simple, purely functional multiplication of two prime elements.
    // Real logic will be added later (e.g., charge handling, commutativity).
    pub fn multiply(a: PrimeElement, b: PrimeElement) -> (PrimeElement, Charge) {
        use Charge::*;
        use PrimeElement::*;
        // Placeholder rule set: identical primes yield positive, different yield negative Chaos.
        if a == b {
            (a, Pos)
        } else {
            (Chaos, Neg)
        }
    }
}

// ──────────────────────────────────────────────────────────────
// Axioms
// ──────────────────────────────────────────────────────────────
pub mod axiom {
    use core::fmt;

    /// Enumeration of core axioms used as guardrails in the Causal Engine.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Axiom {
        /// Zero represents absolute absence and cannot participate in quantity.
        NonZero,
        /// Infinity is a conceptual limit, not an operable value.
        FiniteSets,
    }

    impl fmt::Display for Axiom {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Axiom::NonZero   => write!(f, "Axiom of Non-Zero"),
                Axiom::FiniteSets => write!(f, "Axiom of Finite Sets"),
            }
        }
    }
}
