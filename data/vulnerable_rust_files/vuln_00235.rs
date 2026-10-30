// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0113
// CVE: CVE-2021-45704
// CWE: N/A
// Crate: metrics-util
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/metrics-util
// Extraction Method: repo_search
// GitHub Repo: metrics-rs/metrics
// Commit: 4c0a319
// File: metrics-exporter-prometheus/src/common.rs


/// Represents a set of labels as structured key-value pairs
 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 pub struct LabelSet {
    labels: Vec<(String, String)>,
 }
 
 impl LabelSet {