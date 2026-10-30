// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0035 (FIXED)
// Crate: chunky
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: aeplay/chunky
// Commit: d768521
// File: src/arena.rs


chunks: Vec<Chunk>,
     chunk_size: usize,
     item_size: usize,
    len: Value<usize>,
     storage: Rc<dyn ChunkStorage>
 }
 
         let len = Value::<usize>::load_or_default(ident.sub("len"), 0, Rc::clone(&storage));
         let mut chunks = Vec::new();
 
        let mut item_offset = 0;
        println!("Loading arena {} with len {}", ident.0, *len);

        while item_offset < *len {
            println!("...at offset {}", item_offset);
            chunks.push(storage.load_chunk(ident.sub(item_offset)));
            item_offset += chunk_size / item_size;
         }
 
         Arena {
     pub fn pop_away(&mut self) {
         *self.len -= 1;
         // If possible, remove the last chunk as well
        if *self.len % self.items_per_chunk() == 0 {
             self.storage.forget_chunk(self.chunks.pop().expect("should have chunk left"));
         }
     }