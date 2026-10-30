// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0362
// CVE: N/A
// CWE: N/A
// Crate: alloy-json-abi
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/alloy-json-abi
// Extraction Method: repo_search
// GitHub Repo: alloy-rs/core
// Commit: 7823e9a
// File: crates/dyn-abi/src/eip712/resolver.rs


/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
     pub fn encode_type(&self, name: &str) -> Result<String> {
         let linear = self.linearize(name)?;
         let first = linear.first().unwrap().eip712_encode_type();
 
         // Sort references by name (eip-712 encodeType spec)