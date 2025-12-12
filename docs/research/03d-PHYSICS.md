# 03d-PHYSICS: The Thermodynamics API

*   **File:** `docs/research/03d-PHYSICS.md`
*   **Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)
*   **Author:** Christophe Duy Quang Nguyen (System Ronin)
*   **Context:** Layer 3 Specification (Time, State, Inertia & The Ouroboros Ratchet)
*   **Date:** December 12, 2025
*   **Status:** `v1.3` (The Sovereign Inertia Standard)

> **The Engine of Consequence.**
> *Layer 2 gave us Space. Layer 3 gives us Time and Consequence. This API governs the transition of data between states of matter. It enforces the **Ouroboros Ratchet** (to secure history) and the **Laws of Inertia** (to govern belief updates). It rejects the concept of external "Truth" overwriting the user; instead, it implements a **Sovereign Inertia Model** where beliefs are durable structures that only change through massive Energy (Work) or fade through Entropy (Neglect).*

---

## 1. The Physics Mandate

The functions defined herein enforce the physical constraints of the Digital Universe.

1.  **Irreversibility:** Time moves forward. History cannot be rewritten (Ouroboros).
2.  **Conservation of Energy:** Nothing is free. Every state change consumes an `EnergyToken`.
3.  **Sovereign Inertia:** A concept's resistance to change is proportional to its Mass and its Structural Bonds.
4.  **Entropy:** All mass decays over time unless maintained (Annealed) or structurally supported (Bonded).
5.  **Arbitration:** When Physics cannot resolve a conflict (High Force vs. High Inertia), the system yields to the Owner (Dichotomy).

---

## 2. Core Data Structures

### 2.1 The States of Matter
We explicitly define the four phases of information.

```rust
#[repr(u8)]
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum Phase {
    Crystal = 0, // Solid. Low Temp, High Mass. (Deep Lattice / Immutable Truth)
    Liquid = 1,  // Fluid. Med Temp, Med Mass. (Fovea / Active Thought)
    Gas = 2,     // Volatile. High Temp, Low Mass. (RAM / Context)
    Plasma = 3,  // Error. Max Temp. (Rejection / Logic Failure)
}
```

### 2.2 The Vector Body (The Physical Entity)
Layer 2 defined the `SovereignVector` (the shape). Layer 3 wraps it in a physical body, adding Mass and Bonding data.

```rust
#[repr(C)]
pub struct VectorBody {
    pub shape: SovereignVector, // The Layer 2 Geometry
    pub mass: u32,              // Semantic Weight (Proof of Work)
    pub bonds: u16,             // Structural Support (Number of attached concepts)
    pub temp: u16,              // Volatility (0 = Absolute Truth)
    pub phase: Phase,           // Current State
    pub birth_tick: u64,        // Time of creation (Ouroboros Index)
    pub last_anneal: u64,       // Last time it was reinforced
}
```

---

## 3. The Ouroboros API (Time)

This API manages the flow of time and the "Burning of the Key."

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

## 4. The Thermodynamics API (Inertia & Decay)

This section implements the **Sovereign Inertia Model**.

### 4.1 Inertia Calculation
Resistance is calculated based on Mass and Structure.
*   **Formula:** $R = \frac{Mass \times (1 + Bonds)}{Temp + \epsilon}$
*   **Logic:** A heavy concept ($Mass$) is hard to move. A concept connected to many others ($Bonds$) is even harder to move. A hot concept ($Temp$) is easy to move.

```rust
/// Calculates the Resistance of a body to external force.
fn lvm_resistance(body: &VectorBody) -> u32;
```

### 4.2 The Impact (Phase Transition)
This function attempts to update a belief. It requires an `EnergyToken` to prevent DoS (The Socrates Attack).

```rust
pub const MELTING_THRESHOLD: u32 = 2; // Hysteresis factor (Latent Heat)

/// Applies an informational force to a target vector.
///
/// @param target: The existing belief (e.g., "Earth is Flat").
/// @param force_mass: The weight of the new evidence.
/// @param energy: The cost paid to perform this physical interaction.
///
/// @return Result<Phase, Plasma>
/// - Liquid: If Force > Resistance (Melting).
/// - Crystal: If Force < Resistance (Bouncing).
/// - Plasma: If Energy is insufficient (Starvation).
fn lvm_apply_force(
    target: &mut VectorBody, 
    force_mass: u32,
    energy: EnergyToken
) -> Result<Phase, Plasma>;
```

### 4.3 The Dichotomy (Arbitration)
When Physics fails to resolve a conflict, it escalates to the Sovereign.

```rust
/// Checks if a rejected force was significant enough to warrant user attention.
/// @param target: The belief that resisted change.
/// @param force_mass: The size of the rejected input.
/// @return true if Force > (Resistance / 2).
fn lvm_detect_dichotomy(target: &VectorBody, force_mass: u32) -> bool;
```

### 4.4 Entropy Decay (The Cleanup)
This prevents database bloat ("Heat Death") and implements natural forgetting.

```rust
pub const LVM_DECAY_RATE: u32 = 1;

/// Runs the decay cycle on a body.
/// 1. Reduces Mass by (LVM_DECAY_RATE / Bonds).
///    (Heavily bonded concepts decay slower).
/// 2. If Mass drops below threshold, Phase transitions Crystal -> Gas.
fn lvm_entropy_decay(body: &mut VectorBody);
```

---

## 5. The Sovereignty Oracle

The physics engine must know the hardware reality to enable or disable "God Mode" protections (Antimatter).

```rust
#[repr(u8)]
pub enum SovereigntyTier {
    Guest = 1,     // Software Privacy (No physical erasure, No Secure UI)
    Sovereign = 2, // Hardware Sovereignty (TEE enforced, Antimatter Active)
}

/// Returns the detected hardware security level.
/// WARNING: In Tier 1, this return value can be spoofed by a Rootkit.
/// Only Tier 2 provides hardware-guaranteed truth via the Secure Overlay.
fn lvm_get_tier() -> SovereigntyTier;
```

---

## 6. Implementation Notes for `libcdqn`

*   **Constant Time Physics:** To prevent side-channel attacks (Thermodynamic Leak), `lvm_apply_force` must execute in constant time. If a vector "Bounces" (retains Crystal phase), the CPU must burn wait-cycles equivalent to the time required for a "Melt" operation.
*   **Bonding Check:** `lvm_resistance` assumes the `bonds` count is accurate. Layer 4 (Chemistry) is responsible for incrementing/decrementing this counter when links are created/destroyed.
*   **Boot Protocol:** As per `03a-METAL`, `lvm_tick` must ensure Fovea is zeroed on boot to maintain Ouroboros integrity.

---

### **Appendix: Test Vector Specification**

The correctness of this API will be validated by `docs/validation/03d_physics.check`.

**Example Test Case (`check/inertia.test`):**
```
// Test Case: Bonds increase Inertia against Decay.

// 1. GIVEN: Two identical bodies, one bonded, one isolated.
let mut isolated = VectorBody { mass: 100, bonds: 0, ... };
let mut bonded   = VectorBody { mass: 100, bonds: 10, ... };

// 2. WHEN: We apply 50 cycles of Entropy Decay.
for _ in 0..50 {
    lvm_entropy_decay(&mut isolated);
    lvm_entropy_decay(&mut bonded);
}

// 3. THEN: The isolated body should have lost more mass.
ASSERT isolated.mass < bonded.mass;
```

**License:** Universal Sovereign Source License (USSL) v2.0.
