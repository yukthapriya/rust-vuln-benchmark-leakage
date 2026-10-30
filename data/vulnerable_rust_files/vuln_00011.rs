// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2017-0008
// CVE: N/A
// CWE: N/A
// Crate: serial
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/serial
// Extraction Method: repo_search
// GitHub Repo: dcuddeback/serial-rs
// Commit: c6acde2
// File: src/posix/poll.rs


fn do_poll(fds: &mut Vec<PollFd>, timeout: Duration) -> c_int {
     use std::ptr;
 
     #[repr(C)]
    struct sigset_t;
 
     extern "C" {
         fn ppoll(fds: *mut PollFd, nfds: nfds_t, timeout_ts: *mut self::libc::timespec, sigmask: *const sigset_t) -> c_int;