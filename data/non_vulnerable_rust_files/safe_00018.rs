// REAL FIXED CODE
// RustSec ID: RUSTSEC-2018-0012 (FIXED)
// Crate: orion
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: brycx/orion
// Commit: f63b758
// File: src/errors.rs


let err_one = Error::InvalidInput;
     let err_two = Error::Overflow;
 
    // Tests Debug impl through "{:?}" and Display impl through "{}"
     let err = format!(
         "{:?}:{}",
         UnknownCryptoError::from(err_one),
 fn test_unknown_crypto_from_parseint_error() {
     let err_foreign = "j".parse::<u32>().unwrap_err();
 
    // Tests Debug impl through "{:?}" and Display impl through "{}"
     let err = format!(
         "{:?}:{}",
         UnknownCryptoError::from(err_foreign.clone()),