// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0119 (FIXED)
// Crate: nix
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: nix-rust/nix
// Commit: 41e2f81
// File: src/sys/mod.rs


#[cfg(not(any(
     target_os = "redox",
     target_os = "fuchsia",
    target_os = "solaris",
     target_os = "haiku"
 )))]
 feature! {