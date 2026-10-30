// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0031
// CVE: CVE-2021-28032
// CWE: N/A
// Crate: nano_arena
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/nano_arena
// Extraction Method: repo_search
// GitHub Repo: bennetthardwick/nano-arena
// Commit: 6b83f9d
// File: src/lib.rs


&'a mut self,
         selected: I,
     ) -> Option<(&mut T, ArenaSplit<'a, T>)> {
        if let Some(value) = self.get_mut(selected.borrow()) {
             Some((
                 unsafe { (value as *mut T).as_mut().unwrap() },
                 ArenaSplit {
                    selected: selected.borrow().clone(),
                     arena: self,
                     __type: Default::default(),
                 },
 #[cfg(test)]
 mod tests {
     use super::*;
 
     fn setup_arena() -> (Arena<String>, Idx, Idx, Idx, Idx) {
         let mut arena = Arena::new();
 
         assert_eq!(format!("{:?}", john), "Removed Idx ( 0 )");
     }
 }