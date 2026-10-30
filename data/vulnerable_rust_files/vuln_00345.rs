// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0036
// CVE: N/A
// CWE: N/A
// Crate: tree_magic
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tree_magic
// Extraction Method: repo_search
// GitHub Repo: aahancoc/tree_magic
// Commit: 728d02d
// File: src/fdo_magic/mod.rs


}
     
     pub fn from_u8(b: &[u8]) -> Result<FnvHashMap<MIME, DiGraph<super::MagicRule, u32>>, String> {
        let tuplevec = from_u8_to_tuple_vec(b).to_result().map_err(|e| e.to_string())?;;
         let mut res = FnvHashMap::<MIME, DiGraph<super::MagicRule, u32>>::default();
         
         for x in tuplevec {