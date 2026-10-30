// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0115 (FIXED)
// Crate: zeroize_derive
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: iqlusioninc/crates
// Commit: 9c82152
// File: hkd32/src/path.rs


}
 
 #[cfg(feature = "alloc")]
impl Debug for Component<'_> {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "hkd32::Component")?;