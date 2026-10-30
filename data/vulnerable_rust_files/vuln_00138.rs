// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0127
// CVE: CVE-2020-36447
// CWE: N/A
// Crate: v9
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/v9
// Extraction Method: commit_diff
// GitHub Repo: purpleposeidon/v9
// Commit: 18847c5
// File: src/id.rs


type Run<M> = (Id<M>, Id<M>);
 
pub trait Raw: 'static + Copy + fmt::Debug + Ord + Send + Sync + hash::Hash + serde::Serialize + serde::de::DeserializeOwned + Add<Output=Self> + Sub<Output=Self> {
     fn to_usize(self) -> usize;
     fn from_usize(x: usize) -> Self;
     fn offset(self, d: i8) -> Self;
     const ZERO: Self;
     const LAST: Self;
 }
 mod raw_impl {
     use super::Raw;
     macro_rules! imp {
         ($($ty:ident),*) => {$(
             impl Raw for $ty {
                 #[inline]
                 fn to_usize(self) -> usize { self as usize }