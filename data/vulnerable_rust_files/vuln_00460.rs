// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0408
// CVE: N/A
// CWE: N/A
// Crate: pprof
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pprof
// Extraction Method: repo_search
// GitHub Repo: tikv/pprof-rs
// Commit: 0c52c5f
// File: src/profiler.rs


pub(crate) data: Collector<UnresolvedFrames>,
     sample_counter: i32,
 
     running: bool,
 
     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
 #[derive(Clone)]
 pub struct ProfilerGuardBuilder {
     frequency: c_int,
     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
         ProfilerGuardBuilder {
             frequency: 99,
 
             #[cfg(any(
                 target_arch = "x86_64",
                 target_arch = "aarch64",
         Self { frequency, ..self }
     }
 
     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
                 Err(Error::CreatingError)
             }
             Ok(profiler) => {
                 #[cfg(any(
                     target_arch = "x86_64",
                     target_arch = "aarch64",
         Ok(Profiler {
             data: Collector::new()?,
             sample_counter: 0,
             running: false,
 
             #[cfg(any(
                 target_arch = "x86_64",
                 target_arch = "aarch64",
         }
     }
 
    fn register_signal_handler(&self) -> Result<()> {
         let handler = signal::SigHandler::SigAction(perf_signal_handler);
        let sigaction = signal::SigAction::new(
            handler,
            // SA_RESTART will only restart a syscall when it's safe to do so,
            // e.g. when it's a blocking read(2) or write(2). See man 7 signal.
            signal::SaFlags::SA_SIGINFO | signal::SaFlags::SA_RESTART,
            signal::SigSet::empty(),
        );
        unsafe { signal::sigaction(signal::SIGPROF, &sigaction) }?;

         Ok(())
     }
 
    fn unregister_signal_handler(&self) -> Result<()> {
        let handler = signal::SigHandler::SigIgn;
        unsafe { signal::signal(signal::SIGPROF, handler) }?;

         Ok(())
     }