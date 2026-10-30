// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0018
// CVE: CVE-2022-29185
// CWE: N/A
// Crate: totp-rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/totp-rs
// Extraction Method: repo_search
// GitHub Repo: constantoine/totp-rs
// Commit: d79bac7
// File: src/lib.rs


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
 #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
 pub enum Algorithm {
     SHA1,
     SHA256,
     SHA512,
     #[cfg(feature = "steam")]
     #[cfg_attr(docsrs, doc(cfg(feature = "steam")))]
    /// Steam TOTP token algorithm
     Steam,
 }
 
impl std::default::Default for Algorithm {
     fn default() -> Self {
         Algorithm::SHA1
     }