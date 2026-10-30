// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0035
// CVE: CVE-2020-25576
// CWE: N/A
// Crate: rand_core
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/rand_core
// Extraction Method: repo_search
// GitHub Repo: rust-random/rand
// Commit: a8208d4
// File: rand_core/src/os.rs


}
 }
 
 impl OsError {
     /// Extract the raw OS error code (if this error came from the OS)
     ///
     ///
     /// [1]: https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error
     #[inline]
    pub fn raw_os_error(self) -> Option<i32> {
         self.0.raw_os_error()
     }
 }