// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0113 (FIXED)
// Crate: metrics-util
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: metrics-rs/metrics
// Commit: 4c0a319
// File: metrics-exporter-prometheus/src/common.rs


/// Represents a set of labels as structured key-value pairs
 #[derive(Debug, Clone, PartialEq, Eq, Hash)]
 pub struct LabelSet {
    pub(crate) labels: Vec<(String, String)>,
 }
 
 impl LabelSet {