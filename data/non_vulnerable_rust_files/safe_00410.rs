// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0341 (FIXED)
// Crate: tls-listener
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tmccombs/tls-listener
// Commit: 6c185ff
// File: src/lib.rs


//! TCP or unix domain socket).
 //!
 //! # Features:
 //! - `tokio-net`: Implementations for tokio socket types (default)
 //! - `rt`: Features that depend on the tokio runtime, such as [`SpawningHandshakes`]
//! - `rustls-core`: Support the tokio-rustls backend for tls
//! - `rustls-aws-lc`: Include the aws-lc provider for rustls
//! - `rustls-ring`: Include the ring provider for rustls
//! - `rustls-fips`: Include enabling the "fips" feature for rustls
//! - `native-tls`: support the tokio-native-tls backend for tls
 
 use futures_util::stream::{FuturesUnordered, Stream, StreamExt, TryStreamExt};
 use pin_project_lite::pin_project;