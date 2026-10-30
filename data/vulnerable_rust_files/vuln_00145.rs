// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0141
// CVE: CVE-2020-36461
// CWE: N/A
// Crate: noise_search
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/noise_search
// Extraction Method: repo_search
// GitHub Repo: pipedown/noise
// Commit: 0744ec9
// File: src/aggregates.rs


use std::cmp::Ordering;
 
use json_value::JsonValue;
 
 pub type AggregateInitFun = fn(JsonValue) -> JsonValue;
 pub type AggregateActionFun = fn(&mut JsonValue, JsonValue, Option<&JsonValue>);