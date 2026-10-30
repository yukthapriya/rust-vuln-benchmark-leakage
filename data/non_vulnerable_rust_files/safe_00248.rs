// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0128 (FIXED)
// Crate: rusqlite
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rusqlite/rusqlite
// Commit: fea146e
// File: examples/loadable_extension.rs


/// See <https://sqlite.org/c3ref/load_extension.html> on this function's name and usage.
 /// # Safety
 /// This function is called by SQLite and must be safe to call.
 #[no_mangle]
 pub unsafe extern "C" fn sqlite3_extension_init(
     db: *mut ffi::sqlite3,