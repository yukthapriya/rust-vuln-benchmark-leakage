// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2018-0012
// CVE: CVE-2018-20999
// CWE: N/A
// Crate: orion
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/orion
// Extraction Method: repo_search
// GitHub Repo: brycx/orion
// Commit: f63b758
// File: src/errors.rs


let err_one = Error::InvalidInput;
     let err_two = Error::Overflow;
 
    // Tests Debug impl through "{:?}" and Display impl though "{}"
     let err = format!(
         "{:?}:{}",
         UnknownCryptoError::from(err_one),
 fn test_unknown_crypto_from_parseint_error() {
     let err_foreign = "j".parse::<u32>().unwrap_err();
 
    // Tests Debug impl through "{:?}" and Display impl though "{}"
     let err = format!(
         "{:?}:{}",
         UnknownCryptoError::from(err_foreign.clone()),