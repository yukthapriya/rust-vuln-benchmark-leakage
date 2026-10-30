// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0035 (FIXED)
// Crate: rand_core
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-random/rand
// Commit: a8208d4
// File: rand_core/src/os.rs


}
 }
 
// If [`RawOsError`](https://doc.rust-lang.org/std/io/type.RawOsError.html) is stablized, we can use it.

#[cfg(not(target_os = "uefi"))]
type RawOsError = i32;

#[cfg(target_os = "uefi")]
type RawOsError = usize;

 impl OsError {
     /// Extract the raw OS error code (if this error came from the OS)
     ///
     ///
     /// [1]: https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error
     #[inline]
    pub fn raw_os_error(self) -> Option<RawOsError> {
         self.0.raw_os_error()
     }
 }