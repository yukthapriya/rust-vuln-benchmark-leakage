// REAL FIXED CODE
// RustSec ID: RUSTSEC-2017-0008 (FIXED)
// Crate: serial
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: dcuddeback/serial-rs
// Commit: c6acde2
// File: src/posix/poll.rs


fn do_poll(fds: &mut Vec<PollFd>, timeout: Duration) -> c_int {
     use std::ptr;
 
    use self::libc::{c_void};

     #[repr(C)]
    struct sigset_t {
        __private: c_void
    }
 
     extern "C" {
         fn ppoll(fds: *mut PollFd, nfds: nfds_t, timeout_ts: *mut self::libc::timespec, sigmask: *const sigset_t) -> c_int;