# 03c-PHYSICS: The Thermodynamics API

*   **File:** `docs/research/legacy/03c-PHYSICS.md`
*   **Context:** Layer 3 Specification (Time, Tension, Inertia & The Ouroboros Ratchet)
*   **Date:** December 14, 2025
*   **Status:** `v2.0` (The Harmonic Tension Standard)

> **The Engine of Consequence.**
> *Layer 2 gave us Space. Layer 3 gives us Time and Dynamics. This API governs the transition of data between states of matter (Phase Transitions). It enforces the **Ouroboros Ratchet** (to secure history) and the **Laws of Inertia** (to govern belief updates). It implements the **Discrete Lawvere Laplacian** to resolve Tension (Contradiction) through Harmonic Diffusion.*

---

## 1. The Physics Mandate

The functions defined herein enforce the physical constraints of the Digital Universe.
1.  **Irreversibility:** Time moves forward. History cannot be rewritten (Ouroboros).
2.  **Inertia:** Changing a state requires Force proportional to Mass.
3.  **Logic over Mass:** A geometric contradiction (Deduction) shatters a crystal regardless of its weight (Induction).
4.  **Conservation of Energy:** Operations consume defined resources. Waste heat is generated to ensure constant-time execution.

---

## 2. Core Data Structures

### 2.1 The States of Matter
We explicitly define the four phases of information.

```rust
#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
pub enum Phase {
    Crystal = 0, // Solid. Low Temp, High Mass. (Deep Lattice / Immutable Truth)
    Liquid = 1,  // Fluid. Med Temp, Med Mass. (Fovea / Active Thought)
    Gas = 2,     // Volatile. High Temp, Low Mass. (RAM / Context)
    Plasma = 3,  // Error/Destruction. Max Temp. (Rejection / Logic Failure)
}
```

### 2.2 The Vector Body (The Physical Entity)
Layer 2 defined the `SovereignVector` (the shape). Layer 3 wraps it in a physical body.

```rust
pub struct VectorBody {
    pub shape: SovereignVector, // The Layer 2 Geometry
    pub mass: u32,              // Intrinsic Mass (Rank) + Bond Mass
    pub temp: u16,              // Volatility (0 = Absolute Truth)
    pub phase: Phase,           // Current State
    pub birth_tick: u64,        // Time of creation (Ouroboros Index)
    pub last_anneal: u64,       // Last time it was reinforced
}
```

---

## 3. The Ouroboros API (Time)

This API manages the flow of time and the "Burning of the Key" defined in `02b`.

### 3.1 The Heartbeat
```rust
/// Advances the internal clock of the LVM.
/// 1. Derives the Next Seed (S_t+1) from the Current Seed (S_t).
/// 2. DESTROYS the Old Seed (Secure Erase).
/// 3. Returns the new Tick Index.
///
/// @note In Tier 1 (Guest), Secure Erase is best-effort (memset).
/// @note In Tier 2 (Sovereign), Secure Erase is hardware-enforced (TEE).
fn lvm_tick() -> Result<u64, Plasma>;
```

### 3.2 The Journal
```rust
/// Anchors an event to the immutable timeline.
/// Creates a hash-linked record that cannot be re-ordered.
///
/// @param event_hash: The content to anchor.
/// @return The cryptographic proof of history (The Ratchet State).
fn lvm_anchor_event(event_hash: [u8; 32]) -> [u8; 32];
```

---

## 4. The Thermodynamics API (State & Inertia)

This implementation distinguishes between **Inductive Updates** (Mass vs. Mass) and **Deductive Updates** (Logic vs. Structure).

### 4.1 Inertia Calculation
```rust
/// Calculates the Resistance of a body to Inductive Change.
/// Formula: R = Mass / (Temp + epsilon)
/// A Cold, Heavy object has near-Infinite Resistance.
fn lvm_resistance(body: &VectorBody) -> u32;
```

### 4.2 The Impact (Phase Transition)
This is the core function for updating beliefs. It applies a "Force" (new evidence) to an existing "Body" (current belief).

```rust
pub const MELTING_THRESHOLD: u32 = 2; // Latent Heat (Hysteresis)

/// Applies an informational force to a target vector.
///
/// @param target: The existing belief.
/// @param force_vector: The new evidence.
/// @param force_mass: The weight of the new evidence.
/// @param energy_token: The up-front cost paid to perform this check (The Socrates Mitigation).
///
/// @return Result<Phase, Plasma>
fn lvm_apply_force(
    target: &mut VectorBody, 
    force_vector: &SovereignVector, 
    force_mass: u32,
    energy_token: EnergyToken
) -> Result<Phase, Plasma> {
    
    // 1. DEDUCTIVE CHECK (The Shattering)
    // We check if the new evidence creates a logical paradox (Rank mismatch).
    // This consumes the EnergyToken.
    let rank = lvm_rank_superposition(&target.shape, force_vector);
    
    if rank == 0 {
        // Logical Contradiction Detected.
        // Logic overrides Mass. The target is Falsified.
        target.phase = Phase::Plasma; 
        return Ok(Phase::Plasma); // The Crystal Shatters.
    }

    // 2. INDUCTIVE CHECK (The Melting)
    // If logic holds, we check if the evidence is heavy enough to reshape the belief.
    let resistance = lvm_resistance(target);
    
    // Hysteresis: We need extra energy to overcome Latent Heat.
    if force_mass > (resistance * MELTING_THRESHOLD) {
        // Melting Point Reached.
        target.temp += force_mass as u16; // Absorb Energy
        target.phase = Phase::Liquid;     // Becomes Plastic
        return Ok(Phase::Liquid);
    } else {
        // Insufficient Force. Input Bounces.
        // NOTE: Implementation must burn wait-cycles here to match 
        // the time taken by the 'Liquid' branch (Constant Time).
        return Ok(target.phase);
    }
}
```

### 4.3 Annealing (Harmonic Diffusion)
How does a new truth become permanent? By finding consensus with neighbors.

```rust
/// Reduces the temperature of a body based on successful use.
/// Internally applies the Discrete Laplacian (Bundling) to smooth out local tension.
/// @param body: The vector to anneal.
fn lvm_anneal(body: &mut VectorBody);
```

### 4.4 Entropy Decay (Background Radiation)
This prevents "Heat Death" (Database Bloat) by cleaning up unused concepts.

```rust
pub const LVM_BACKGROUND_TEMP: u16 = 10; // Ambient Noise

/// Runs the decay cycle. Called during idle ticks.
/// 1. If Temp < Background, Body absorbs heat (Temp increases).
/// 2. If Mass is low and Temp is high, Phase -> Gas.
/// 3. If in Gas phase for too long, Phase -> Plasma (Garbage Collection).
fn lvm_entropy_decay(body: &mut VectorBody, current_tick: u64);
```

---

## 5. The Sovereignty Oracle

The physics engine must know the hardware reality to enable or disable "God Mode" protections.

```rust
#[repr(u8)]
pub enum SovereigntyTier {
    Guest = 1,     // Software Privacy (No physical erasure, No Secure UI)
    Sovereign = 2, // Hardware Sovereignty (TEE enforced, Antimatter Active)
}

/// Returns the detected hardware security level.
fn lvm_get_tier() -> SovereigntyTier;
```

---

## 6. Implementation Notes for `libcdqn`

*   **Logic vs. Mass:** The implementation of `lvm_apply_force` must prioritize the Matroid check. This ensures that a single valid counter-example can overthrow a massive dogma (The "Black Swan" effect).
*   **Constant Time Physics:** To prevent side-channel attacks, the `lvm_apply_force` function must execute in constant time regardless of whether the state changes or bounces. Dummy cycles must be burned if the resistance check fails.
*   **Concurrency:** `lvm_tick` is a global barrier operation. It requires a write lock on the Ouroboros state.

---

### Appendix: Test Vector Specification

The correctness of this API will be validated by `docs/validation/03c_physics.check`.

**Example Test Case (`check/shatter.test`):**
```
// Test Case: Deductive Logic shatters Inductive Mass.
// 1. GIVEN: A massive "Flat Earth" crystal.
// 2. WHEN: We apply a Logical Contradiction (Rank 0 superposition).
// 3. THEN: The target must SHATTER (Plasma), not Resist.
```

**License:** Universal Sovereign Source License (USSL) v2.0.
