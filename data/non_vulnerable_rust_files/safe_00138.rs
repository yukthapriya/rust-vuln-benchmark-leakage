// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0127 (FIXED)
// Crate: v9
// Type: race_condition (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: purpleposeidon/v9
// Commit: 18847c5
// File: src/id.rs


type Run<M> = (Id<M>, Id<M>);
 
pub trait Raw
where
    Self: 'static + Send + Sync,
    Self: Ord + Copy + fmt::Debug + hash::Hash,
    Self: serde::Serialize + serde::de::DeserializeOwned,
    Self: Add<Output=Self> + Sub<Output=Self>,
    Self: self::raw_impl::Sealed,
{
     fn to_usize(self) -> usize;
     fn from_usize(x: usize) -> Self;
     fn offset(self, d: i8) -> Self;
     const ZERO: Self;
     const LAST: Self;
 }
 mod raw_impl {
    /// Forbid non-primitives from being put into a SyncRef.
    pub trait Sealed {}
     use super::Raw;
     macro_rules! imp {
         ($($ty:ident),*) => {$(
            impl Sealed for $ty {}
             impl Raw for $ty {
                 #[inline]
                 fn to_usize(self) -> usize { self as usize }