// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0080 (FIXED)
// Crate: miow
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: yoshuawuyts/miow
// Commit: 86e9087
// File: src/handle.rs


pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
         let mut bytes = 0;
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
         crate::cvt(unsafe {
             WriteFile(
                 self.0,
                 buf.as_ptr() as *const _,
                 len,
                 &mut bytes,
                std::ptr::null_mut(),
             )
         })?;
         Ok(bytes as usize)
     }
 
     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
         let mut bytes = 0;
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
         crate::cvt(unsafe {
             ReadFile(
                 self.0,
                 buf.as_mut_ptr() as *mut _,
                 len,
                 &mut bytes,
                std::ptr::null_mut(),
             )
         })?;
         Ok(bytes as usize)
         overlapped: *mut OVERLAPPED,
         wait: BOOL,
     ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
         let res = crate::cvt({
             ReadFile(
                 self.0,
         overlapped: *mut OVERLAPPED,
         wait: BOOL,
     ) -> io::Result<Option<usize>> {
        let len = cmp::min(buf.len(), u32::MAX as usize) as u32;
         let res = crate::cvt({
             WriteFile(
                 self.0,