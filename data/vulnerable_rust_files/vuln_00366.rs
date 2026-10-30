// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0071
// CVE: CVE-2023-49092
// CWE: N/A
// Crate: rsa
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rsa
// Extraction Method: repo_search
// GitHub Repo: RustCrypto/RSA
// Commit: 6e3e33e
// File: src/algorithms/pss.rs


let h0 = hash.finalize_reset();
 
     // 14. If H = H', output "consistent." Otherwise, output "inconsistent."
    if (salt_valid & h0.ct_eq(h)).into() {
         Ok(())
     } else {
         Err(Error::Verification)