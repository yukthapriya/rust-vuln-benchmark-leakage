// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0035
// CVE: CVE-2020-36433
// CWE: N/A
// Crate: chunky
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/chunky
// Extraction Method: repo_search
// GitHub Repo: aeplay/chunky
// Commit: d768521
// File: src/arena.rs


chunks: Vec<Chunk>,
     chunk_size: usize,
     item_size: usize,
    pub len: Value<usize>,
     storage: Rc<dyn ChunkStorage>
 }
 
         let len = Value::<usize>::load_or_default(ident.sub("len"), 0, Rc::clone(&storage));
         let mut chunks = Vec::new();
 
        for i in 0..*len {
            chunks.push(storage.load_chunk(ident.sub(i)));
         }
 
         Arena {
     pub fn pop_away(&mut self) {
         *self.len -= 1;
         // If possible, remove the last chunk as well
        if *self.len + self.items_per_chunk() < self.chunks.len() * self.items_per_chunk() {
             self.storage.forget_chunk(self.chunks.pop().expect("should have chunk left"));
         }
     }