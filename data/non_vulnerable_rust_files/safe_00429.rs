// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0363 (FIXED)
// Crate: sqlx
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: launchbadge/sqlx
// Commit: e8384f2
// File: examples/postgres/axum-social-with-tests/src/http/error.rs


/// A SQLx call returned an error.
     ///
     /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about database internals.
     #[error("an internal database error occurred")]
     Sqlx(#[from] sqlx::Error),