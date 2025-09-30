// File Path: chronos_ai/src/cdu/cdu_model.rs

use std::collections::HashMap;

// --- 1. State Definition (The User Intent) ---
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct State {
    pub task_type: String, // e.g., "COUNT"
    pub start_val: i32,
    pub end_val: i32,
    pub constraint: Option<String>, // e.g., Some("ODD_FINAL")
}

// --- 2. Hybrid Logical Clock (HLC) ---
#[derive(Debug, Clone, Default)]
pub struct HybridLogicalClock {
    // L-component: Increments on every logical step/CDU reference.
    pub logical_steps: u64, 
    // P-component: Increments based on a physical time source.
    pub physical_time_ticks: u64, 
}

impl HybridLogicalClock {
    // Simple heuristic for multi-objective scoring (to be refined later)
    pub fn to_composite_score(&self) -> f64 {
        (self.logical_steps as f64) * 100.0 + (self.physical_time_ticks as f64) * 0.01
    }
}

// --- 3. Optimization Metrics ---
#[derive(Debug, Clone, Default)]
pub struct PathMetrics {
    pub avg_execution_time_ticks: f64, 
    pub avg_cdu_steps: f64,      
    pub dpath_encounters_during_discovery: u32, 
}

// --- 4. CDU Role Definition ---
#[derive(Debug, Clone)]
pub enum CduRole {
    Axiom,      // Atomic, fundamental logic (e.g., N => N+1)
    Goal,       // The final requirement of a user intent
    BPath,      // A verified, successful sequence of CDUs
    DPath,      // A known, failed sequence of CDUs
    GPath,      // The *optimal* BPath/Playbook for a given State/Goal
}

// --- 5. CDU Content Definition ---
#[derive(Debug, Clone)]
pub enum CduContent {
    AxiomContent {
        logic_form: String,
    },
    GoalContent {
        intent_state: State, 
    },
    PathContent {
        sequence: Vec<String>, // List of tokens of referenced CDUs
        metrics: PathMetrics,
    },
    DecisionMapContent { // For GPath: The Playbook
        map: HashMap<State, String>, // State -> Next_CDU_Token
        metrics: PathMetrics,
    },
}

// --- 6. The Main CDU Structure ---
#[derive(Debug, Clone)]
pub struct Cdu {
    pub token: String, // Content-Address (Hash)
    pub role: CduRole,
    pub content: CduContent,
}

// BaDaaS License Attribution
// This file is part of the Chronos AI project, licensed under BaDaaS.
