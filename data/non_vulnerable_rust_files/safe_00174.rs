// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0016 (FIXED)
// Crate: ms3d
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: andrewhickman/ms3d
// Commit: 599313b
// File: src/read.rs


impl<R: io::Read> BufReadExact for IoReader<R> {
     fn buf_read_exact(&mut self, len: usize) -> io::Result<&[u8]> {
        self.buf.resize(len, 0);
        self.rdr.read_exact(self.buf.as_mut_slice())?;
        Ok(self.buf.as_slice())
     }
 }