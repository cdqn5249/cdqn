// BaDaaS License
// File Path: crates/cdqn_core/src/time/mod.rs
//
// Copyright (c) 2025 Christophe Duy Quang Nguyen
//
// This file is part of the CDQN (Causal Data Query Nodes) ecosystem.
//
// Licensed under the BaDaaS (Build and Develop as a Service) License.
// See LICENSE-BaDaaS.md in the project root for terms.
//
// AI-Mediated Access: All interactions with the CDQN network must be
// mediated by an AI entity (ProxyAgent) as per the BaDaaS license.

//! # Time Module
//!
//! Provides primitives for distributed time and causality.
//!
//! This module contains the `HybridLogicalClock` (HLC), a crucial component
//! for establishing a verifiable causal order of events across a distributed
//! network of sovereign nodes.

#![forbid(unsafe_code)]

/// The Hybrid Logical Clock (HLC) implementation.
pub mod hlc;
