// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0015
// CVE: N/A
// CWE: N/A
// Crate: pty
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pty
// Extraction Method: repo_search
// GitHub Repo: hibariya/pty-rs
// Commit: 6f48d9a
// File: src/fork/pty/master/mod.rs


mod err;
 
use ::{libc, ffi};
 
 use ::descriptor::Descriptor;
 
     /// fd is provided, to the real UID and real GID of the calling thread.
     pub fn grantpt(&self) -> Result<libc::c_int> {
         unsafe {
            match ffi::grantpt(self.as_raw_fd()) {
                 -1 => Err(MasterError::GrantptError),
                 c => Ok(c),
             }
     /// Unlock the slave pty associated with the master to which fd refers.
     pub fn unlockpt(&self) -> Result<libc::c_int> {
         unsafe {
            match ffi::unlockpt(self.as_raw_fd()) {
                 -1 => Err(MasterError::UnlockptError),
                 c => Ok(c),
             }
 
     /// Returns a pointer to a static buffer, which will be overwritten on
     /// subsequent calls.
    pub fn ptsname(&self) -> Result<*const libc::c_schar> {
         unsafe {
            match ffi::ptsname(self.as_raw_fd()) {
                 c if c.is_null() => Err(MasterError::PtsnameError),
                 c => Ok(c),
             }
 impl io::Read for Master {
     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
         unsafe {
            match ffi::read(self.as_raw_fd(), buf.as_mut_ptr(), buf.len()) {
                 -1 => Ok(0),
                 len => Ok(len as usize),
             }
 impl io::Write for Master {
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
         unsafe {
            match ffi::write(self.as_raw_fd(), buf.as_ptr(), buf.len()) {
                 -1 => Err(io::Error::last_os_error()),
                 ret => Ok(ret as usize),
             }