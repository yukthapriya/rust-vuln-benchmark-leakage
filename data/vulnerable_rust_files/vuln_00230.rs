// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0107
// CVE: CVE-2021-45698
// CWE: N/A
// Crate: ckb
// Type: unknown
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