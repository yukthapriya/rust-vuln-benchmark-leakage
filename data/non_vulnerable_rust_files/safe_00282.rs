// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0034 (FIXED)
// Crate: pkcs11
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mheese/rust-pkcs11
// Commit: 48a15f8
// File: src/types.rs


pub fn get_ck_ulong(&self) -> Result<CK_ULONG, Error> {
         self.available_value()?;
        let value: CK_ULONG = unsafe { (self.pValue as *const CK_ULONG).read_unaligned() };
        Ok(value)
     }
 
     #[allow(clippy::trivially_copy_pass_by_ref)]
 
     pub fn get_ck_long(&self) -> Result<CK_LONG, Error> {
         self.available_value()?;
        let value: CK_LONG = unsafe { (self.pValue as *const CK_LONG).read_unaligned() };
        Ok(value)
     }
 
     pub fn with_biginteger(mut self, val: &[u8]) -> Self {
 
     pub fn get_date(&self) -> Result<CK_DATE, Error> {
         self.available_value()?;
        let value: CK_DATE = unsafe { (self.pValue as *const CK_DATE).read_unaligned() };
        Ok(value)
     }
 
     /// Check if the value contained by this attribute is invalid or unavailable.