// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0078 (FIXED)
// Crate: net2
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: deprecrated/net2-rs
// Commit: 6081dff
// File: src/socket.rs


sin_family: c::AF_INET as c::sa_family_t,
             sin_port: addr.port().to_be(),
             sin_addr,
            #[cfg(not(target_os = "haiku"))]
             sin_zero: [0; 8],
            #[cfg(target_os = "haiku")]
            sin_zero: [0; 24],
             #[cfg(any(
                 target_os = "dragonfly",
                 target_os = "freebsd",
                 target_os = "ios",
                 target_os = "macos",
                 target_os = "netbsd",
                target_os = "openbsd",
                target_os = "haiku",
             ))]
             sin_len: 0,
         },
                 target_os = "ios",
                 target_os = "macos",
                 target_os = "netbsd",
                target_os = "openbsd",
                target_os = "haiku",
             ))]
             sin6_len: 0,
             #[cfg(any(target_os = "solaris", target_os = "illumos"))]