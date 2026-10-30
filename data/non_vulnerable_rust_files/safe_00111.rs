// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0097 (FIXED)
// Crate: xcb
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-x-bindings/rust-xcb
// Commit: da83097
// File: src/base.rs


use std::fmt::{self, Display, Formatter};
 use std::marker::{self, PhantomData};
 use std::mem;
use std::os::fd::{IntoRawFd, OwnedFd};
 use std::os::unix::prelude::{AsRawFd, RawFd};
 use std::ptr;
 use std::result;
     /// Connects to an X server, given the open socket fd and the
     /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
     /// If the connection should be unauthenticated, `auth_info` must be `None`.
    #[deprecated(note = "unsound API, use `connect_with_fd` instead")]
    #[doc(hidden)]
     pub fn connect_to_fd(fd: RawFd, auth_info: Option<AuthInfo<'_>>) -> ConnResult<Self> {
        #[allow(deprecated)]
         Self::connect_to_fd_with_extensions(fd, auth_info, &[], &[])
     }
 
     ///
     /// # Panics
     /// Panics if one of the mandatory extension is not present.
    #[deprecated(note = "unsound API, use `connect_with_fd_and_extensions` instead")]
    #[doc(hidden)]
     pub fn connect_to_fd_with_extensions(
         fd: RawFd,
         auth_info: Option<AuthInfo<'_>>,
         conn.has_error().map(|_| conn)
     }
 
    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    pub fn connect_with_fd(fd: OwnedFd, auth_info: Option<AuthInfo<'_>>) -> ConnResult<Self> {
        Self::connect_with_fd_and_extensions(fd, auth_info, &[], &[])
    }

    /// Connects to the X server with an open socket file descriptor and optional authentification info.
    ///
    /// Extension data specified by `mandatory` and `optional` is cached to allow
    /// the resolution of events and errors in these extensions.
    ///
    /// Connects to an X server, given the open socket fd and the
    /// `auth_info`. The file descriptor `fd` is bidirectionally connected to an X server.
    /// If the connection should be unauthenticated, `auth_info` must be `None`.
    ///
    /// # Panics
    /// Panics if one of the mandatory extension is not present.
    pub fn connect_with_fd_and_extensions(
        fd: OwnedFd,
        auth_info: Option<AuthInfo<'_>>,
        mandatory: &[Extension],
        optional: &[Extension],
    ) -> ConnResult<Self> {
        let mut auth_info = auth_info.map(|auth_info| {
            let auth_name = CString::new(auth_info.name).unwrap();
            let auth_data = CString::new(auth_info.data).unwrap();

            let auth_info = xcb_auth_info_t {
                namelen: auth_name.as_bytes().len() as _,
                name: auth_name.as_ptr() as *mut _,
                datalen: auth_data.as_bytes().len() as _,
                data: auth_data.as_ptr() as *mut _,
            };
            // return the strings too otherwise they would drop
            (auth_info, auth_name, auth_data)
        });

        let ai_ptr = if let Some(auth_info) = auth_info.as_mut() {
            &mut auth_info.0 as *mut _
        } else {
            ptr::null_mut()
        };

        let fd = fd.into_raw_fd();

        let conn = unsafe {
            let conn = xcb_connect_to_fd(fd, ai_ptr);
            check_connection_error(conn)?;

            Self::from_raw_conn_and_extensions(conn, mandatory, optional)
        };

        conn.has_error().map(|_| conn)
    }

     /// Connects to the X server, using an authorization information.
     ///
     /// Connects to the X server specified by `display_name`, using the