// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0077
// CVE: N/A
// CWE: N/A
// Crate: memmap
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/memmap
// Extraction Method: repo_search
// GitHub Repo: danburkert/memmap-rs
// Commit: 83f3802
// File: src/lib.rs


/// Returns the configured length, or the length of the provided file.
     fn get_len(&self, file: &File) -> Result<usize> {
         self.len.map(Ok).unwrap_or_else(|| {
            let len = file.metadata()?.len();
             if len > (usize::MAX as u64) {
                 return Err(Error::new(
                     ErrorKind::InvalidData,
                    "file length overflows usize",
                 ));
             }
            Ok((len - self.offset) as usize)
         })
     }
 
         let len = 5432;
         file.set_len(offset + len as u64).unwrap();
 
        // Check auto-length mmap. The file length exceeds 4GiB, so this fails on 32-bit platforms.
        #[cfg(target_pointer_width = "64")]
        {
            let mmap = unsafe { MmapOptions::new().offset(offset).map_mut(&file).unwrap() };
            assert_eq!(len, mmap.len());
        }
 
        // Check explicit length mmap. This should succeed on any platform.
         let mut mmap = unsafe {
             MmapOptions::new()
                 .offset(offset)