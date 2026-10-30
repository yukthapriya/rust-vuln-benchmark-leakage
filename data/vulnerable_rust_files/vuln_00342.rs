// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0033
// CVE: N/A
// CWE: N/A
// Crate: borsh
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/borsh
// Extraction Method: repo_search
// GitHub Repo: near/borsh-rs
// Commit: e01e16d
// File: borsh/src/de/mod.rs


}
 }
 
 impl<T> BorshDeserialize for Cow<'_, T>
 where
     T: ToOwned + ?Sized,