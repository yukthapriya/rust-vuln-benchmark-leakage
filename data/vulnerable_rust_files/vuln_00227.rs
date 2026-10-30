// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0099
// CVE: N/A
// CWE: N/A
// Crate: cosmos_sdk
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cosmos_sdk
// Extraction Method: repo_search
// GitHub Repo: cosmos/cosmos-rust
// Commit: 9e08e91
// File: cosmrs/src/tx/builder.rs


/// Add a non-critical extension option.
     pub fn non_critical_extension_option(&mut self, option: impl Into<Any>) -> &mut Self {
        self.body.extension_options.push(option.into());
         self
     }