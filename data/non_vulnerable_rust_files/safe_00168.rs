// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0008 (FIXED)
// Crate: bra
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Enet4/bra-rs
// Commit: 6fe222c
// File: src/greedy.rs


/// [`std::io::BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
 /// [`new`]: ./struct.GreedyAccessReader.html#method.new
 /// [`with_capacity`]: ./struct.GreedyAccessReader.html#method.with_capacity
#[derive(Debug, Clone)]
 pub struct GreedyAccessReader<R> {
     inner: R,
     buf: Vec<u8>,
         }
 
         let b = self.buf.len();
        self.buf.resize(self.buf.capacity(), 0);
        let buf = &mut self.buf[b..];
        let o = self.inner.read(buf)?;
 
        // truncate to exclude non-written portion
        self.buf.truncate(b + o);
 
        Ok(&self.buf[self.consumed..])
     }
 
     fn consume(&mut self, amt: usize) {