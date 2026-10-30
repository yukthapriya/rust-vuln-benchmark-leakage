// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0126 (FIXED)
// Crate: signal-simple
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: kitsuneninetails/signal-rust
// Commit: 4cd38ff
// File: src/signal.rs


signal_internal(signum, int_sig_handler, flags)
 }
 
#[cfg(target_os = "linux")]
 fn signal_internal(signum: int, sig_handler: libc::size_t, flags: Option<int>) {
     let mut sigset: libc::sigset_t = unsafe { uninitialized() };
     let _ = unsafe { libc::sigemptyset(&mut sigset as *mut libc::sigset_t) };
     unsafe { libc::sigaction(signum, &siga as *const libc::sigaction, &mut oldact as *mut libc::sigaction) };
 }
 
#[cfg(target_os = "macos")]
fn signal_internal(signum: int, sig_handler: libc::size_t, flags: Option<int>) {
    let mut sigset: libc::sigset_t = unsafe { uninitialized() };
    let _ = unsafe { libc::sigemptyset(&mut sigset as *mut libc::sigset_t) };

    let mut siga = unsafe { uninitialized::<libc::sigaction>() };
    let mut oldact = unsafe { uninitialized::<libc::sigaction>() };

    siga.sa_sigaction = sig_handler;
    siga.sa_mask = sigset;
    siga.sa_flags = flags.map(|x| { x - libc::SA_SIGINFO }).unwrap_or(libc::SA_ONSTACK | libc::SA_RESTART);

    unsafe { libc::sigaction(signum, &siga as *const libc::sigaction, &mut oldact as *mut libc::sigaction) };
}

 pub fn default(signum: int) {
     let int_sig_handler: libc::size_t = unsafe { transmute(libc::SIG_DFL) };
     signal_internal(signum, int_sig_handler, None)