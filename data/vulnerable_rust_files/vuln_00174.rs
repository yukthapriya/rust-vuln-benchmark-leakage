// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0016
// CVE: CVE-2021-26952
// CWE: N/A
// Crate: ms3d
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ms3d
// Extraction Method: repo_search
// GitHub Repo: andrewhickman/ms3d
// Commit: 599313b
// File: src/read.rs


impl<R: io::Read> BufReadExact for IoReader<R> {
     fn buf_read_exact(&mut self, len: usize) -> io::Result<&[u8]> {
        unsafe {
            self.buf.reserve(len);
            let slice = self.buf.get_unchecked_mut(..len);
            self.rdr.read_exact(slice)?;
            Ok(slice)
        }
     }
 }