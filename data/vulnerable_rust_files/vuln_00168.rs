// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0008
// CVE: CVE-2021-25905
// CWE: N/A
// Crate: bra
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/bra
// Extraction Method: repo_search
// GitHub Repo: Enet4/bra-rs
// Commit: 6fe222c
// File: src/greedy.rs


/// [`std::io::BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
 /// [`new`]: ./struct.GreedyAccessReader.html#method.new
 /// [`with_capacity`]: ./struct.GreedyAccessReader.html#method.with_capacity
 pub struct GreedyAccessReader<R> {
     inner: R,
     buf: Vec<u8>,
         }
 
         let b = self.buf.len();
        let buf = unsafe {
            // safe because it's within the buffer's limits
            // and we won't be reading uninitialized memory
            std::slice::from_raw_parts_mut(
                self.buf.as_mut_ptr().add(b),
                self.buf.capacity() - b)
        };
 
        match self.inner.read(buf) {
            Ok(o) => {
                unsafe {
                    // reset the size to include the written portion,
                    // safe because the extra data is initialized
                    self.buf.set_len(b + o);
                }
 
                Ok(&self.buf[self.consumed..])
            }
            Err(e) => Err(e),
        }
     }
 
     fn consume(&mut self, amt: usize) {