// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0036 (FIXED)
// Crate: internment
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: droundy/internment
// Commit: 6c5ec80
// File: src/boxedmap.rs


.0
             .iter()
             .map(|(n, v)| {
                core::mem::size_of::<P>()
                     + v.deep_size_of_children(context)
                    + core::mem::size_of_val(v)
             })
             .sum::<usize>();
         pointers + heap_memory
     for HashMap<Box<P>, V>
 {
     fn deep_size_of_children(&self, context: &mut deepsize::Context) -> usize {
        let cap = self.0.capacity();
        let pointers = cap * core::mem::size_of::<Box<P>>();
         let heap_memory = self
             .0
             .iter()
             .map(|(n, v)| {
                 (**n).deep_size_of_children(context)
                     + core::mem::size_of_val(&**n)
                     + v.deep_size_of_children(context)
                    + core::mem::size_of_val(v)
             })
             .sum::<usize>();
         pointers + heap_memory
         self.0.insert(x, v);
     }
     #[inline]
    pub fn len(&self) -> usize {
         self.0.len()
     }
     #[allow(dead_code)] // maybe unused without `deepsize` feature