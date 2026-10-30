// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0094 (FIXED)
// Crate: reffers
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: diwic/reffers-rs
// Commit: 5100c34
// File: src/arc.rs


impl_arc_all!(RefMut, None);
 
 
 /// An Rc reference which has immutable access to the inner value.
 ///
     fn clone(&self) -> Self { self.0.get_ref().unwrap() }
 }
 
 /// A strong reference without access to the inner value.
 ///
 /// To get immutable/mutable access, you need to use the
     fn clone(&self) -> Self { self.0.get_strong().unwrap() }
 }
 
 /// A weak reference without access to the inner value.
 ///
 /// To get immutable/mutable access, you need to use the
     impl_ref_all!();
 }
 
 
 
 #[test]