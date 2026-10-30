// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0039 (FIXED)
// Crate: simple-slab
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: nathansizemore/simple-slab
// Commit: 5e0524c
// File: benches/index/src/main.rs


}
 
 fn slab_get(find_list: &Vec<usize>, results: &mut Vec<BenchResult>) {
    let mut buf = slab::Slab::<usize>::with_capacity(NUM_ELEMS);
     for x in 0..NUM_ELEMS {
         let _ = buf.insert(x);
     }