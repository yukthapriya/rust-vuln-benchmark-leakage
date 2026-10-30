// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0040 (FIXED)
// Crate: users
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ogham/rust-users
// Commit: 5765d03
// File: src/cache.rs


//! value. In this case, switching to `&mut self` would only allow for one user
 //! to be read at a time!
 //!
//! ```no_run
//! use users::{Users, Groups, UsersCache};
//!
//! let mut cache = UsersCache::new();
 //!
//! let uid   = cache.get_current_uid();                          // OK...
//! let user  = cache.get_user_by_uid(uid).unwrap();              // OK...
//! let group = cache.get_group_by_gid(user.primary_group_id());  // No!
 //! ```
 //!
 //! When we get the `user`, it returns an optional reference (which we unwrap)
 //! we’re just trying the same trick as earlier. A simplified implementation of
 //! a user cache lookup would look something like this:
 //!
//! ```text
 //! fn get_user_by_uid(&self, uid: uid_t) -> Option<&User> {
 //!     let users = self.users.borrow_mut();
 //!     users.get(uid)