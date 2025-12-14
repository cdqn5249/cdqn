# 03d-CHEMISTRY: The Interaction API

*   **File:** `docs/research/03d-CHEMISTRY.md`
*   **Context:** Layer 4 Specification (Valency, Bonding, Linear Logic & The Quantale Economy)
*   **Date:** December 14, 2025
*   **Status:** `v2.0` (The Metabolist Standard)

> **The Engine of Transaction.**
> *Layer 3 defined the object. Layer 4 defines the relationship. This API implements the **Quantale Economy**: a system where every interaction is a chemical reaction that consumes resources (Energy) to produce results. It enforces **Mass-based Valency** to prevent structural spam and implements a **Bio-Mimetic Metabolism** where energy is derived strictly from Work (Mechanical) and Attention (Symbiotic).*

---

## 1. The Chemistry Mandate

The functions defined herein enforce the **Logic of Interaction**.

1.  **Linear Logic:** Resources are consumed, not copied. (The "No-Cloning" implementation).
2.  **Metabolic Origin:** Energy is generated only by **Work** (CPU) or **Symbiosis** (Human). It cannot be printed via Trade.
3.  **Valency Constraints:** **$Slots \propto Mass$**. Heavy concepts support more connections.
4.  **Endothermic Physics:** Both Creating and Breaking bonds consume Energy.
5.  **Competitive Displacement:** A Strong Bond can displace a Weak Bond if the Valency is full.
6.  **Circular Economy:** Waste Plasma can be recycled into Energy at a loss.

---

## 2. Core Data Structures

### 2.1 The Bond (The Edge)
A connection between two Vector Bodies.

```rust
#[repr(u8)]
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum BondType {
    Covalent = 2, // Strongest. Identity. High Creation Cost.
    Ionic = 1,    // Medium. Session/Access.
    Hydrogen = 0, // Weakest. Context. Low Creation Cost.
}

#[repr(C)]
pub struct Bond {
    pub target: u64,      // The ID (Ouroboros Index) of the other atom
    pub bond_type: BondType,
    pub strength: u8,     // Current Tensile Strength
}
```

### 2.2 The Energy Token (The Currency)
We add metadata to the token to track its source (Provenance).

```rust
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EnergySource {
    Mechanical = 0, // CPU Work
    Symbiotic = 1,  // Human Attention (High Value)
    Recycled = 3,   // Waste Recovery
}

#[repr(C)]
pub struct EnergyToken {
    pub value: u32,
    pub source: EnergySource,
}
```

---

## 3. The Quantale Oracle (The Mint)

This API converts Activity into Economic Potential.

### 3.1 Mechanical Mint (CPU)
```rust
/// Converts CPU cycles (Work) into Energy.
/// Base rate. Slow but steady.
/// @param cycles: The proof of work provided by the caller.
fn lvm_mint_mechanical(cycles: u32) -> EnergyToken;
```

### 3.2 Symbiotic Mint (Human-in-the-Loop)
This creates the advantage for the Local Node (Phone).
*   **Trigger:** The `AgentBuilder` (UI) sends a "Confirmation Signal".
*   **Coefficient:** Boosted Rate (e.g., 10x CPU rate).
*   **Security:** Signed by the TEE/Totem to prevent script-spoofing (as per `03a`).

```rust
/// Mint energy reward for user assistance.
/// @param proof: Cryptographic proof of human interaction (Totem/TEE).
fn lvm_mint_symbiotic(proof: InteractionProof) -> EnergyToken;
```

### 3.3 Recycling (The Loop)
*   **Source:** Waste Heat / Broken Bonds.
*   **Efficiency:** ~20% recovery (Second Law of Thermodynamics applies).

```rust
/// Recovers energy from destroyed structures.
/// @param waste_plasma: The debris from broken bonds.
fn lvm_recycle_waste(waste_plasma: Plasma) -> EnergyToken;
```

---

## 4. The Valency API & Structural Efficiency

This defines the shape of the graph and the cost of maintaining it.

```rust
/// Calculates the maximum number of bonds a Body can hold.
/// Logic: Valency = Floor( Log2(Body.Mass) ) + 1.
/// A Mass of 1 has Valency 1. A Mass of 1024 has Valency 11.
fn lvm_get_max_valency(body: &VectorBody) -> u16;

/// Calculates the MAINTENANCE COST (Decay Rate) of a body.
/// Logic:
/// 1. Base Cost = Mass * Standard_Decay.
/// 2. Efficiency Bonus: If Bonds are COVALENT (Stable), cost is reduced.
///    Highly connected crystals effectively "freeze" and cost almost nothing to keep.
fn lvm_get_decay_rate(body: &VectorBody) -> u32;
```

---

## 5. The Reaction API (The Transaction Engine)

This implements the chemical equation: $A + B + Energy \to C + Waste$.

### 5.1 Bonding (Synthesis & Displacement)
Forms a link. Consumes Energy. If slots are full, it checks if it can displace a weaker bond.

```rust
/// Attempts to bond 'a' to 'b'.
///
/// @param energy: The cost paid. Must be > BondType Base Cost.
/// @return Result:
///   - Ok: Bond formed.
///   - Plasma::ValencyFull: If slots are full AND new bond is too weak to displace.
///   - Plasma::Starvation: If energy < Cost.
fn lvm_react_bond(
    a: &mut VectorBody, 
    b: &mut VectorBody, 
    bond_type: BondType, 
    energy: EnergyToken
) -> Result<Success, Plasma>;
```
**Displacement Logic:**
If `current_bonds == max_valency`:
1.  Find the weakest existing bond (e.g., Hydrogen).
2.  If `New_Bond > Weakest_Bond`: Break Weakest (generate Waste), Install New.
3.  Else: Reject New (Bounce).

### 5.2 Dissolution (Analysis)
Breaks a link. **Crucially, this consumes Energy.**

```rust
/// Breaks a bond.
/// @param energy: Must be > Bond.strength.
/// @return Result<Plasma>: The Plasma is returned to the caller to be
///         fed into lvm_recycle_waste() if desired.
fn lvm_react_break(
    a: &mut VectorBody, 
    target_id: u64, 
    energy: EnergyToken
) -> Result<Plasma, Plasma>;
```

### 5.3 Catalysis (Logic Execution)
Enables reusable logic without destroying the rule vector.

```rust
/// Applies a Transformation Rule (Catalyst) to a set of Reagents.
/// Catalyst is Read-Only but suffers slight entropy wear.
fn lvm_react_catalysis(
    reagents: &mut [VectorBody], 
    catalyst: &VectorBody, 
    out: &mut VectorBody
) -> Result<Phase, Plasma>;
```

### 5.4 Lazy Access (The Snap)
Retrieves a bond. If the target has decayed (Layer 3), the bond snaps.

```rust
/// Follows a bond to retrieve the target.
/// If target is missing/dead, the bond is removed from 'origin' automatically.
fn lvm_follow_bond(
    origin: &mut VectorBody, 
    bond_index: u16
) -> Result<&VectorBody, Plasma>;
```

---

## 6. Implementation Notes for `libcdqn`

*   **Symbiotic Security:** `lvm_mint_symbiotic` is the high-value target for hackers. It must be strictly gated by **Layer 1 TEE Constraints**. On Tier 1 (Guest), this minting rate must be capped to prevent "Click Farm" abuse.
*   **Thread Safety:** `lvm_react_bond` modifies two bodies simultaneously. Requires careful locking or message passing to prevent deadlocks.
*   **Genesis Concepts:** The Kernel initializes with a "Starter Pack" of pre-annealed vectors so the Agent doesn't start at zero mass.

---

### Appendix: Test Vector Specification

The correctness of this API will be validated by `docs/validation/03d_chemistry.check`.

**Example Test Case (`check/displacement.test`):**
```
// Test Case: Strong Bond displaces Weak Bond.

// 1. GIVEN: Atom with Mass 1 (Valency 1).
let mut atom = VectorBody { mass: 1, ... }; 
// It already has a Hydrogen (Weak) bond.
lvm_react_bond(&mut atom, &mut weak_target, Hydrogen, token);

// 2. WHEN: We try to add a Covalent (Strong) bond.
let result = lvm_react_bond(&mut atom, &mut strong_target, Covalent, high_energy_token);

// 3. THEN: The Hydrogen bond is destroyed. The Covalent bond is attached.
ASSERT atom.bonds[0].type == Covalent;
```

**License:** Universal Sovereign Source License (USSL) v2.0.
