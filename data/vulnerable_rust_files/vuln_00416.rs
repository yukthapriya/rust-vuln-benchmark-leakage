// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0349
// CVE: CVE-2024-35186
// CWE: N/A
// Crate: gix-worktree
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/gix-worktree
// Extraction Method: repo_search
// GitHub Repo: Byron/gitoxide
// Commit: e93e84e
// File: gix-filter/src/driver/apply.rs


///
     /// ### Deviation
     ///
    /// If a long running process returns the 'abort' status after receiving the data, it will be removed similarly to how `git` does it.
     /// However, if it returns an unsuccessful error status later, it will not be removed, but reports the error only.
     /// If any other non-'error' status is received, the process will be stopped. But that doesn't happen if such a status is received
     /// after reading the filtered result.
         }
     }
 
    /// Like [`apply()]`[Self::apply()], but use `delay` to determine if the filter result may be delayed or not.
     ///
    /// Poll [`list_delayed_paths()`][Self::list_delayed_paths()] until it is empty and query the available paths again.
     /// Note that even though it's possible, the API assumes that commands aren't mixed when delays are allowed.
     pub fn apply_delayed<'a>(
         &'a mut self,
     ) -> Result<Option<MaybeDelayed<'a>>, Error> {
         match self.maybe_launch_process(driver, operation, ctx.rela_path)? {
             Some(Process::SingleFile { mut child, command }) => {
                std::io::copy(src, &mut child.stdin.take().expect("configured"))?;
                 Ok(Some(MaybeDelayed::Immediate(Box::new(ReadFilterOutput {
                     inner: child.stdout.take(),
                     child: driver.required.then_some((child, command)),
                 }))))
             }
             Some(Process::MultiFile { client, key }) => {
     Immediate(Box<dyn std::io::Read + 'a>),
 }
 
 /// A utility type to facilitate streaming the output of a filter process.
 struct ReadFilterOutput {
     inner: Option<std::process::ChildStdout>,
     /// The child is present if we need its exit code to be positive.
     child: Option<(std::process::Child, std::process::Command)>,
 }
 
 pub(crate) fn handle_io_err(err: &std::io::Error, running: &mut HashMap<BString, process::Client>, process: &BStr) {
     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
         match self.inner.as_mut() {
             Some(inner) => {
                let num_read = inner.read(buf)?;
                 if num_read == 0 {
                     self.inner.take();
                     if let Some((mut child, cmd)) = self.child.take() {
                         let status = child.wait()?;
                         if !status.success() {