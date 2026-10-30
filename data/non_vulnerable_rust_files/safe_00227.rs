// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0099 (FIXED)
// Crate: cosmos_sdk
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: cosmos/cosmos-rust
// Commit: 9e08e91
// File: cosmrs/src/tx/builder.rs


/// Add a non-critical extension option.
     pub fn non_critical_extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.non_critical_extension_options.push(option.into());
         self
     }