// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0052
// CVE: N/A
// CWE: N/A
// Crate: os_socketaddr
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/os_socketaddr
// Extraction Method: repo_search
// GitHub Repo: a-ba/os_socketaddr
// Commit: 3162c0e
// File: src/lib.rs


use std::str::FromStr;
 
 #[cfg(not(target_os = "windows"))]
use libc::{sockaddr, sockaddr_in, sockaddr_in6, socklen_t, AF_INET, AF_INET6};
 
 #[cfg(target_os = "windows")]
use winapi::{
    shared::{
        ws2def::{AF_INET, AF_INET6, SOCKADDR as sockaddr, SOCKADDR_IN as sockaddr_in},
        ws2ipdef::SOCKADDR_IN6_LH as sockaddr_in6,
    },
    um::ws2tcpip::socklen_t,
 };
 
 /// A type for handling platform-native socket addresses (`struct sockaddr`)
     sa6: sockaddr_in6,
 }
 
 #[allow(dead_code)]
 impl OsSocketAddr {
     /// Create a new empty socket address (all bytes initialised to 0)