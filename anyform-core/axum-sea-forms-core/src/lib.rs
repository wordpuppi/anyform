//! Shared types for anyform.
//!
//! This crate contains types that are shared between the server library
//! and the WASM client, including:
//!
//! - [`ConditionRule`] - Dynamic step/field visibility conditions
//! - [`ConditionOp`] - Condition operators (eq, neq, gt, gte, lt, lte, etc.)

mod condition;

pub use condition::{ConditionOp, ConditionRule};
