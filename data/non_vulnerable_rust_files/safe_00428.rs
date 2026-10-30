// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0362 (FIXED)
// Crate: alloy-json-abi
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: alloy-rs/core
// Commit: 7823e9a
// File: crates/dyn-abi/src/eip712/resolver.rs


/// <https://eips.ethereum.org/EIPS/eip-712#definition-of-encodetype>
     pub fn encode_type(&self, name: &str) -> Result<String> {
         let linear = self.linearize(name)?;
        if linear.is_empty() {
            return Err(Error::missing_type(name));
        }
         let first = linear.first().unwrap().eip712_encode_type();
 
         // Sort references by name (eip-712 encodeType spec)