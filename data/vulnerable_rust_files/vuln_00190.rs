// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0041
// CVE: CVE-2021-29932
// CWE: N/A
// Crate: parse_duration
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/parse_duration
// Extraction Method: repo_search
// GitHub Repo: zeta12ti/parse_duration
// Commit: 68a724f
// File: src/lib.rs


//! Any value without a unit will produce an error, unless *only* that unit is passed
 //! (besides non-word characters).
 //! In that case, the value is interpreted as seconds.
//! For example, `".:++++]][][[][15[]][][]:}}}}"` would be interpreted as seconds.
 //!
 //! If the same unit is specified more than once, the sum of the values is used.
 //!
 //! - years
 //!
 //! Years are defined using the average over 400 years in the Gregorian calendar.
//! As such a year is equivalent to 365.2425 days. A month is defined as one twelfth of a year.
 //!
 //! Abbreviations for each of these units are accepted.
 //! The general rule is that any initial segment of the full name is accepted as long as it's not
 //! use std::time::Duration;
 //!
 //! // Full names may be used
//! assert_eq!(parse("10 days 1 nanoseconds 15 years").unwrap(), Duration::new(474_218_280, 1));
 //! // or very short names
//! assert_eq!(parse("10d1n15y").unwrap(), Duration::new(474_218_280, 1));
 //! ```
 //!
 //! # Values
 //!
 //! Negatives are allowed, but the negative sign must be directly adjacent to the value:
 //! `"-15 seconds"`, not `"- 15 seconds"`.
//! When using negative values, the sum must end up non-negative, since `Durations` are positive
 //! durations.
 //!
 //! Decimals are accurate up to nanosecond precision.