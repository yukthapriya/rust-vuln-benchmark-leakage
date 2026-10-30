// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0077 (FIXED)
// Crate: memmap
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: danburkert/memmap-rs
// Commit: 83f3802
// File: src/lib.rs


/// Returns the configured length, or the length of the provided file.
     fn get_len(&self, file: &File) -> Result<usize> {
         self.len.map(Ok).unwrap_or_else(|| {
            let len = file.metadata()?.len() - self.offset;
             if len > (usize::MAX as u64) {
                 return Err(Error::new(
                     ErrorKind::InvalidData,
                    "memory map length overflows usize",
                 ));
             }
            Ok(len as usize)
         })
     }
 
         let len = 5432;
         file.set_len(offset + len as u64).unwrap();
 
        // Check inferred length mmap.
        let mmap = unsafe { MmapOptions::new().offset(offset).map_mut(&file).unwrap() };
        assert_eq!(len, mmap.len());
 
        // Check explicit length mmap.
         let mut mmap = unsafe {
             MmapOptions::new()
                 .offset(offset)