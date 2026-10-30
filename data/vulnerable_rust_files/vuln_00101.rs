// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0080
// CVE: CVE-2020-35921
// CWE: N/A
// Crate: miow
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/miow
// Extraction Method: repo_search
// GitHub Repo: yoshuawuyts/miow
// Commit: 86e9087
// File: src/handle.rs


pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
         let mut bytes = 0;
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
         crate::cvt(unsafe {
             WriteFile(
                 self.0,
                 buf.as_ptr() as *const _,
                 len,
                 &mut bytes,
                0 as *mut _,
             )
         })?;
         Ok(bytes as usize)
     }
 
     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
         let mut bytes = 0;
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
         crate::cvt(unsafe {
             ReadFile(
                 self.0,
                 buf.as_mut_ptr() as *mut _,
                 len,
                 &mut bytes,
                0 as *mut _,
             )
         })?;
         Ok(bytes as usize)
         overlapped: *mut OVERLAPPED,
         wait: BOOL,
     ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
         let res = crate::cvt({
             ReadFile(
                 self.0,
         overlapped: *mut OVERLAPPED,
         wait: BOOL,
     ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), <u32>::max_value() as usize) as u32;
         let res = crate::cvt({
             WriteFile(
                 self.0,