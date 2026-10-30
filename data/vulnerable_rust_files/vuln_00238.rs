// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0116
// CVE: N/A
// CWE: N/A
// Crate: arrow
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/arrow
// Extraction Method: repo_search
// GitHub Repo: apache/arrow-rs
// Commit: af7a07c
// File: arrow-cast/src/display.rs


/// this example requires the `prettyprint` feature.
 ///
 /// ```rust
 /// use std::fmt::Write;
 /// use arrow_array::{cast::AsArray, Array, Int32Array};
 /// use arrow_cast::display::{ArrayFormatter, ArrayFormatterFactory, DisplayIndex, FormatOptions, FormatResult};
 ///        &my_batches,
 ///        &FormatOptions::new().with_formatter_factory(Some(&MyFormatters {}))
 /// );
 /// ```
 pub trait ArrayFormatterFactory: Debug + Send + Sync {
     /// Creates a new [`ArrayFormatter`] for the given [`Array`] and an optional [`Field`]. If the