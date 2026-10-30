// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0126
// CVE: CVE-2020-36446
// CWE: N/A
// Crate: signal-simple
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/signal-simple
// Extraction Method: repo_search
// GitHub Repo: kitsuneninetails/signal-rust
// Commit: 4cd38ff
// File: src/signal.rs


signal_internal(signum, int_sig_handler, flags)
 }
 
 fn signal_internal(signum: int, sig_handler: libc::size_t, flags: Option<int>) {
     let mut sigset: libc::sigset_t = unsafe { uninitialized() };
     let _ = unsafe { libc::sigemptyset(&mut sigset as *mut libc::sigset_t) };
     unsafe { libc::sigaction(signum, &siga as *const libc::sigaction, &mut oldact as *mut libc::sigaction) };
 }
 
 pub fn default(signum: int) {
     let int_sig_handler: libc::size_t = unsafe { transmute(libc::SIG_DFL) };
     signal_internal(signum, int_sig_handler, None)