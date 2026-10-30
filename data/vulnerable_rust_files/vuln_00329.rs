// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0003
// CVE: CVE-2023-22742
// CWE: N/A
// Crate: libgit2-sys
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/libgit2-sys
// Extraction Method: repo_search
// GitHub Repo: rust-lang/git2-rs
// Commit: a2b4f0b
// File: src/merge.rs


/// Do not write the REUC extension on the generated index
     pub fn skip_reuc(&mut self, skip: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_FAIL_ON_CONFLICT as u32, skip)
     }
 
     /// If the commits being merged have multiple merge bases, do not build a