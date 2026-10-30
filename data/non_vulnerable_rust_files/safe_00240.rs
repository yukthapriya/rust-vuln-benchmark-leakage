// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0118 (FIXED)
// Crate: arrow
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: apache/arrow-rs
// Commit: af7a07c
// File: arrow-cast/src/display.rs


/// this example requires the `prettyprint` feature.
 ///
 /// ```rust
/// # #[cfg(feature = "prettyprint")]{
 /// use std::fmt::Write;
 /// use arrow_array::{cast::AsArray, Array, Int32Array};
 /// use arrow_cast::display::{ArrayFormatter, ArrayFormatterFactory, DisplayIndex, FormatOptions, FormatResult};
 ///        &my_batches,
 ///        &FormatOptions::new().with_formatter_factory(Some(&MyFormatters {}))
 /// );
/// # }
 /// ```
 pub trait ArrayFormatterFactory: Debug + Send + Sync {
     /// Creates a new [`ArrayFormatter`] for the given [`Array`] and an optional [`Field`]. If the