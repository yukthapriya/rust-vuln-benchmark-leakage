// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0079 (FIXED)
// Crate: socket2
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang/socket2-rs
// Commit: 4af7d3d
// File: src/sys/unix.rs


#[cfg(all(feature = "all", any(target_os = "android", target_os = "linux")))]
     pub fn tcp_notsent_lowat(&self) -> io::Result<u32> {
         unsafe {
            getsockopt::<c_int>(self.as_raw(), libc::IPPROTO_TCP, libc::TCP_NOTSENT_LOWAT)
                 .map(|lowat| lowat as u32)
         }
     }