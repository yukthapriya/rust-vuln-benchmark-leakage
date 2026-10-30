// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0128
// CVE: CVE-2021-45713
// CWE: N/A
// Crate: rusqlite
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rusqlite
// Extraction Method: repo_search
// GitHub Repo: rusqlite/rusqlite
// Commit: fea146e
// File: examples/loadable_extension.rs


/// See <https://sqlite.org/c3ref/load_extension.html> on this function's name and usage.
 /// # Safety
 /// This function is called by SQLite and must be safe to call.
#[expect(clippy::not_unsafe_ptr_arg_deref)]
 #[no_mangle]
 pub unsafe extern "C" fn sqlite3_extension_init(
     db: *mut ffi::sqlite3,