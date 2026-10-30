// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0079
// CVE: CVE-2020-35919
// CWE: N/A
// Crate: socket2
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/socket2
// Extraction Method: repo_search
// GitHub Repo: rust-lang/socket2-rs
// Commit: 4af7d3d
// File: src/sys/unix.rs


#[cfg(all(feature = "all", any(target_os = "android", target_os = "linux")))]
     pub fn tcp_notsent_lowat(&self) -> io::Result<u32> {
         unsafe {
            getsockopt::<Bool>(self.as_raw(), libc::IPPROTO_TCP, libc::TCP_NOTSENT_LOWAT)
                 .map(|lowat| lowat as u32)
         }
     }