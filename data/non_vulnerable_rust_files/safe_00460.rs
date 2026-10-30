// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0408 (FIXED)
// Crate: pprof
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tikv/pprof-rs
// Commit: 0c52c5f
// File: src/profiler.rs


pub(crate) data: Collector<UnresolvedFrames>,
     sample_counter: i32,
 
    old_sigaction: Option<signal::SigAction>,
     running: bool,
 
    #[cfg(feature = "frame-pointer")]
    on_stack: bool,

     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
 #[derive(Clone)]
 pub struct ProfilerGuardBuilder {
     frequency: c_int,

    #[cfg(feature = "frame-pointer")]
    on_stack: bool,

     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
         ProfilerGuardBuilder {
             frequency: 99,
 
            #[cfg(feature = "frame-pointer")]
            on_stack: false,

             #[cfg(any(
                 target_arch = "x86_64",
                 target_arch = "aarch64",
         Self { frequency, ..self }
     }
 
    #[cfg(feature = "frame-pointer")]
    /// Sets whether to use an alternate signal stack via `SA_ONSTACK`.
    ///
    /// This is only available and only works correctly when the `frame-pointer` feature is enabled.
    ///
    /// The `backtrace-rs` unwinder ignores the signal context and unwinds the current stack. Using
    /// an alternate stack with it would produce meaningless results. The `frame-pointer` unwinder,
    /// however, uses the provided `ucontext` to correctly walk the original application stack.
    ///
    /// This should be enabled when the profiler is used in an environment
    /// with small stacks (e.g., inside a Go program) to prevent stack overflow.
    pub fn on_stack(self, on_stack: bool) -> Self {
        Self { on_stack, ..self }
    }

     #[cfg(any(
         target_arch = "x86_64",
         target_arch = "aarch64",
                 Err(Error::CreatingError)
             }
             Ok(profiler) => {
                #[cfg(feature = "frame-pointer")]
                {
                    profiler.on_stack = self.on_stack;
                }

                 #[cfg(any(
                     target_arch = "x86_64",
                     target_arch = "aarch64",
         Ok(Profiler {
             data: Collector::new()?,
             sample_counter: 0,
            old_sigaction: None,
             running: false,
 
            #[cfg(feature = "frame-pointer")]
            on_stack: false,

             #[cfg(any(
                 target_arch = "x86_64",
                 target_arch = "aarch64",
         }
     }
 
    fn register_signal_handler(&mut self) -> Result<()> {
         let handler = signal::SigHandler::SigAction(perf_signal_handler);
        // SA_RESTART will only restart a syscall when it's safe to do so,
        // e.g. when it's a blocking read(2) or write(2). See man 7 signal.
        let flags = signal::SaFlags::SA_SIGINFO | signal::SaFlags::SA_RESTART;
        #[cfg(feature = "frame-pointer")]
        let flags = if self.on_stack {
            // SA_ONSTACK will deliver the signal on an alternate stack. This is crucial
            // to prevent a stack overflow if the signal arrives at a thread with
            // a small stack, which is common when use pprof-rs in Go runtimes.
            flags | signal::SaFlags::SA_ONSTACK
        } else {
            flags
        };
        let sigaction = signal::SigAction::new(handler, flags, signal::SigSet::empty());
        let old_action = unsafe { signal::sigaction(signal::SIGPROF, &sigaction) }?;
        self.old_sigaction = Some(old_action);
         Ok(())
     }
 
    fn unregister_signal_handler(&mut self) -> Result<()> {
        if let Some(old_action) = self.old_sigaction.take() {
            unsafe { signal::sigaction(signal::SIGPROF, &old_action) }?;
        }
         Ok(())
     }