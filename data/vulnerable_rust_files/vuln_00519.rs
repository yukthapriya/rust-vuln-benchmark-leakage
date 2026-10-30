// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0051
// CVE: N/A
// CWE: N/A
// Crate: xcb
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/xcb
// Extraction Method: repo_search
// GitHub Repo: rust-x-bindings/rust-xcb
// Commit: da83097
// File: src/base.rs


use std::fmt::{self, Display, Formatter};
 use std::marker::{self, PhantomData};
 use std::mem;
 use std::os::unix::prelude::{AsRawFd, RawFd};
 use std::ptr;
 use std::result;
     /// Connects to an X server, given the open socket fd and the
     /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
     /// If the connection should be unauthenticated, `auth_info` must be `None`.
     pub fn connect_to_fd(fd: RawFd, auth_info: Option<AuthInfo<'_>>) -> ConnResult<Self> {
         Self::connect_to_fd_with_extensions(fd, auth_info, &[], &[])
     }
 
     ///
     /// # Panics
     /// Panics if one of the mandatory extension is not present.
     pub fn connect_to_fd_with_extensions(
         fd: RawFd,
         auth_info: Option<AuthInfo<'_>>,
         conn.has_error().map(|_| conn)
     }
 
     /// Connects to the X server, using an authorization information.
     ///
     /// Connects to the X server specified by `display_name`, using the