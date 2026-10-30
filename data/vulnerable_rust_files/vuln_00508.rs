// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0040
// CVE: CVE-2025-5791
// CWE: N/A
// Crate: users
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/users
// Extraction Method: repo_search
// GitHub Repo: ogham/rust-users
// Commit: 5765d03
// File: src/cache.rs


//! value. In this case, switching to `&mut self` would only allow for one user
 //! to be read at a time!
 //!
//! ```norun
//! let mut cache = UsersCache::empty_cache();
 //!
//! let uid   = cache.get_current_uid();                     // OK...
//! let user  = cache.get_user_by_uid(uid).unwrap()          // OK...
//! let group = cache.get_group_by_gid(user.primary_group);  // No!
 //! ```
 //!
 //! When we get the `user`, it returns an optional reference (which we unwrap)
 //! we’re just trying the same trick as earlier. A simplified implementation of
 //! a user cache lookup would look something like this:
 //!
//! ```norun
 //! fn get_user_by_uid(&self, uid: uid_t) -> Option<&User> {
 //!     let users = self.users.borrow_mut();
 //!     users.get(uid)