// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0034
// CVE: N/A
// CWE: N/A
// Crate: pkcs11
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pkcs11
// Extraction Method: repo_search
// GitHub Repo: mheese/rust-pkcs11
// Commit: 48a15f8
// File: src/types.rs


pub fn get_ck_ulong(&self) -> Result<CK_ULONG, Error> {
         self.available_value()?;
        Ok(unsafe { mem::transmute_copy(&*self.pValue) })
     }
 
     #[allow(clippy::trivially_copy_pass_by_ref)]
 
     pub fn get_ck_long(&self) -> Result<CK_LONG, Error> {
         self.available_value()?;
        Ok(unsafe { mem::transmute_copy(&*self.pValue) })
     }
 
     pub fn with_biginteger(mut self, val: &[u8]) -> Self {
 
     pub fn get_date(&self) -> Result<CK_DATE, Error> {
         self.available_value()?;
        Ok(unsafe { mem::transmute_copy(&*self.pValue) })
     }
 
     /// Check if the value contained by this attribute is invalid or unavailable.