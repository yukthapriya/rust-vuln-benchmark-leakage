// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0021 (FIXED)
// Crate: gix-features
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: GitoxideLabs/gitoxide
// Commit: e93e84e
// File: gix-filter/src/driver/apply.rs


///
     /// ### Deviation
     ///
    /// If a long-running process returns the 'abort' status after receiving the data, it will be removed similarly to how `git` does it.
     /// However, if it returns an unsuccessful error status later, it will not be removed, but reports the error only.
     /// If any other non-'error' status is received, the process will be stopped. But that doesn't happen if such a status is received
     /// after reading the filtered result.
         }
     }
 
    /// Like [`apply()`](Self::apply()), but use `delay` to determine if the filter result may be delayed or not.
     ///
    /// Poll [`list_delayed_paths()`](Self::list_delayed_paths()) until it is empty and query the available paths again.
     /// Note that even though it's possible, the API assumes that commands aren't mixed when delays are allowed.
     pub fn apply_delayed<'a>(
         &'a mut self,
     ) -> Result<Option<MaybeDelayed<'a>>, Error> {
         match self.maybe_launch_process(driver, operation, ctx.rela_path)? {
             Some(Process::SingleFile { mut child, command }) => {
                // To avoid deadlock when the filter immediately echoes input to output (like `cat`),
                // we need to write to stdin and read from stdout concurrently. If we write all data
                // to stdin before reading from stdout, and the pipe buffer fills up, both processes
                // will block: the filter blocks writing to stdout (buffer full), and we block writing
                // to stdin (waiting for the filter to consume data).
                //
                // Solution: Read all data into a buffer, then spawn a thread to write it to stdin
                // while we can immediately read from stdout.
                let mut input_data = Vec::new();
                std::io::copy(src, &mut input_data)?;

                let stdin = child.stdin.take().expect("configured");
                let write_thread = WriterThread::write_all_in_background(input_data, stdin)?;

                 Ok(Some(MaybeDelayed::Immediate(Box::new(ReadFilterOutput {
                     inner: child.stdout.take(),
                     child: driver.required.then_some((child, command)),
                    write_thread: Some(write_thread),
                 }))))
             }
             Some(Process::MultiFile { client, key }) => {
     Immediate(Box<dyn std::io::Read + 'a>),
 }
 
/// A helper to manage writing to stdin on a separate thread to avoid deadlock.
struct WriterThread {
    handle: Option<std::thread::JoinHandle<std::io::Result<()>>>,
}

impl WriterThread {
    /// Spawn a thread that will write all data from `data` to `stdin`.
    fn write_all_in_background(data: Vec<u8>, mut stdin: std::process::ChildStdin) -> std::io::Result<Self> {
        let handle = std::thread::Builder::new()
            .name("gix-filter-stdin-writer".into())
            .stack_size(128 * 1024)
            .spawn(move || {
                use std::io::Write;
                stdin.write_all(&data)?;
                // Explicitly drop stdin to close the pipe and signal EOF to the child
                drop(stdin);
                Ok(())
            })?;

        Ok(Self { handle: Some(handle) })
    }

    /// Wait for the writer thread to complete and return any error that occurred.
    fn join(&mut self) -> std::io::Result<()> {
        let Some(handle) = self.handle.take() else {
            return Ok(());
        };
        handle.join().map_err(|panic_info| {
            let msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                format!("Writer thread panicked: {s}")
            } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                format!("Writer thread panicked: {s}")
            } else {
                "Writer thread panicked while writing to filter stdin".to_string()
            };
            std::io::Error::other(msg)
        })?
    }
}

impl Drop for WriterThread {
    fn drop(&mut self) {
        // Best effort join on drop.
        if let Err(_err) = self.join() {
            gix_trace::debug!(err = %_err, "Failed to join writer thread during drop");
        }
    }
}

 /// A utility type to facilitate streaming the output of a filter process.
 struct ReadFilterOutput {
     inner: Option<std::process::ChildStdout>,
     /// The child is present if we need its exit code to be positive.
     child: Option<(std::process::Child, std::process::Command)>,
    /// The thread writing to stdin, if any. Must be joined when reading is done.
    write_thread: Option<WriterThread>,
 }
 
 pub(crate) fn handle_io_err(err: &std::io::Error, running: &mut HashMap<BString, process::Client>, process: &BStr) {
     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
         match self.inner.as_mut() {
             Some(inner) => {
                let num_read = match inner.read(buf) {
                    Ok(n) => n,
                    Err(e) => {
                        // On read error, ensure we join the writer thread before propagating the error.
                        // This is expected to finish with failure as well as it should fail to write
                        // to the process which now fails to produce output (that we try to read).
                        if let Some(mut write_thread) = self.write_thread.take() {
                            // Try to join but prioritize the original read error
                            if let Err(_thread_err) = write_thread.join() {
                                gix_trace::debug!(thread_err = %_thread_err, read_err = %e, "write to stdin error during failed read");
                            }
                        }
                        return Err(e);
                    }
                };

                 if num_read == 0 {
                     self.inner.take();

                    // Join the writer thread first to ensure all data has been written
                    // and that resources are freed now.
                    if let Some(mut write_thread) = self.write_thread.take() {
                        write_thread.join()?;
                    }

                     if let Some((mut child, cmd)) = self.child.take() {
                         let status = child.wait()?;
                         if !status.success() {