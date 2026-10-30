// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0012 (FIXED)
// Crate: cdr
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: hrektts/cdr-rs
// Commit: 4345ef4
// File: src/de.rs


fn read_vec(&mut self) -> Result<Vec<u8>> {
         let len: u32 = de::Deserialize::deserialize(&mut *self)?;
        let mut buf = vec![0_u8; len as usize];
         self.read_size(u64::from(len))?;
         self.reader.read_exact(&mut buf[..])?;
         Ok(buf)