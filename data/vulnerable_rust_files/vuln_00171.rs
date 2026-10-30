// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0012
// CVE: CVE-2021-26305
// CWE: N/A
// Crate: cdr
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/cdr
// Extraction Method: repo_search
// GitHub Repo: hrektts/cdr-rs
// Commit: 4345ef4
// File: src/de.rs


fn read_vec(&mut self) -> Result<Vec<u8>> {
         let len: u32 = de::Deserialize::deserialize(&mut *self)?;
        let mut buf = Vec::with_capacity(len as usize);
        buf.resize(len as usize, 0);
         self.read_size(u64::from(len))?;
         self.reader.read_exact(&mut buf[..])?;
         Ok(buf)