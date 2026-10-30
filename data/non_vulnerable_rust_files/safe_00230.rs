// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0107 (FIXED)
// Crate: ckb
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: nervosnetwork/ckb
// Commit: 16bf91b
// File: ckb-bin/src/subcommand/replay.rs


.template(
                 "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
             )
            .expect("Failed to set progress bar template")
             .progress_chars("#>-"),
     );
     let switch = if full_verification {