// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0115
// CVE: CVE-2021-45706
// CWE: N/A
// Crate: zeroize_derive
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/zeroize_derive
// Extraction Method: repo_search
// GitHub Repo: iqlusioninc/crates
// Commit: 9c82152
// File: hkd32/src/path.rs


}
 
 #[cfg(feature = "alloc")]
impl<'a> Debug for Component<'a> {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "hkd32::Component")?;