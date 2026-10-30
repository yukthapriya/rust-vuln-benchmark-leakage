// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0108
// CVE: CVE-2021-45699
// CWE: N/A
// Crate: ckb
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/ckb
// Extraction Method: repo_search
// GitHub Repo: nervosnetwork/ckb
// Commit: 16bf91b
// File: ckb-bin/src/subcommand/replay.rs


.template(
                 "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
             )
             .progress_chars("#>-"),
     );
     let switch = if full_verification {