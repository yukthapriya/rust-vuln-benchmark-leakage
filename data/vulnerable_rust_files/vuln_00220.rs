// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0090
// CVE: CVE-2021-45688
// CWE: N/A
// Crate: ash
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/ash
// Extraction Method: repo_search
// GitHub Repo: MaikKlein/ash
// Commit: ccfd77e
// File: ash/src/vk.rs


/// # Panics
     /// If `next` contains a pointer chain of its own, this function will panic.  Call `unsafe`
     /// [`Self::extend()`] to insert this chain instead.
    fn push<T: Extends<Self> + TaggedStructure<'a>>(mut self, next: &'a mut T) -> Self {
         // SAFETY: All implementers of `TaggedStructure` are required to have the `BaseOutStructure` layout
         let slf_base = unsafe { &mut *<*mut _>::cast::<BaseOutStructure<'_>>(&mut self) };
         // SAFETY: All implementers of `T: TaggedStructure` are required to have the `BaseOutStructure` layout
     ///
     /// The last struct in this chain (i.e. the one where `p_next` is `NULL`) must be writable
     /// memory, as its `p_next` field will be updated with the value of `self.p_next`.
    unsafe fn extend<T: Extends<Self> + TaggedStructure<'a>>(mut self, next: &'a mut T) -> Self {
         // `next` here can contain a pointer chain. This means that we must correctly attach he head
         // to the root and the tail to the rest of the chain For example:
         //