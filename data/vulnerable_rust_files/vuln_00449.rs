// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0390
// CVE: N/A
// CWE: N/A
// Crate: minitrace
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/minitrace
// Extraction Method: repo_search
// GitHub Repo: tikv/minitrace-rust
// Commit: f34070b
// File: minitrace-futures/src/lib.rs


/// An extension trait for [`futures::Stream`] that provides tracing instrument adapters.
 pub trait StreamExt: futures::Stream + Sized {
    /// Binds a [`Span`] to the [`Stream`] that continues to record until the stream is **finished**.
     ///
    /// In addition, it sets the span as the local parent at every poll so that [`minitrace::local::LocalSpan`]
    /// becomes available within the future. Internally, it calls [`Span::set_local_parent`] when
    /// the executor polls it.
     ///
     /// # Examples:
     ///
 pub trait SinkExt<Item>: futures::Sink<Item> + Sized {
     /// Binds a [`Span`] to the [`Sink`] that continues to record until the sink is **closed**.
     ///
    /// In addition, it sets the span as the local parent at every poll so that [`minitrace::local::LocalSpan`]
    /// becomes available within the future. Internally, it calls [`Span::set_local_parent`] when
    /// the executor polls it.
     ///
     /// # Examples:
     ///