// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0141 (FIXED)
// Crate: noise_search
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: pipedown/noise
// Commit: 0744ec9
// File: src/aggregates.rs


use std::cmp::Ordering;
 
use crate::json_value::JsonValue;
 
 pub type AggregateInitFun = fn(JsonValue) -> JsonValue;
 pub type AggregateActionFun = fn(&mut JsonValue, JsonValue, Option<&JsonValue>);