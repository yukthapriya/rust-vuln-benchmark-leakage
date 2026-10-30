// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0341
// CVE: CVE-2024-28854
// CWE: N/A
// Crate: tls-listener
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tls-listener
// Extraction Method: repo_search
// GitHub Repo: tmccombs/tls-listener
// Commit: 6c185ff
// File: src/lib.rs


//! TCP or unix domain socket).
 //!
 //! # Features:
//! - `rustls`: Support the tokio-rustls backend for tls (default)
//! - `native-tls`: support the tokio-native-tls backend for tls
 //! - `tokio-net`: Implementations for tokio socket types (default)
 //! - `rt`: Features that depend on the tokio runtime, such as [`SpawningHandshakes`]
 
 use futures_util::stream::{FuturesUnordered, Stream, StreamExt, TryStreamExt};
 use pin_project_lite::pin_project;