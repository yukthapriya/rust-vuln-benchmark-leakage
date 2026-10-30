// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0119
// CVE: CVE-2021-45707
// CWE: N/A
// Crate: nix
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/nix
// Extraction Method: repo_search
// GitHub Repo: nix-rust/nix
// Commit: 41e2f81
// File: src/sys/mod.rs


#[cfg(not(any(
     target_os = "redox",
     target_os = "fuchsia",
    solarish,
     target_os = "haiku"
 )))]
 feature! {