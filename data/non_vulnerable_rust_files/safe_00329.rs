// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0003 (FIXED)
// Crate: libgit2-sys
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang/git2-rs
// Commit: a2b4f0b
// File: src/merge.rs


/// Do not write the REUC extension on the generated index
     pub fn skip_reuc(&mut self, skip: bool) -> &mut MergeOptions {
        self.flag(raw::GIT_MERGE_SKIP_REUC as u32, skip)
     }
 
     /// If the commits being merged have multiple merge bases, do not build a