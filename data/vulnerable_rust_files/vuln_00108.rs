// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0094
// CVE: CVE-2020-36203
// CWE: N/A
// Crate: reffers
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/reffers
// Extraction Method: repo_search
// GitHub Repo: diwic/reffers-rs
// Commit: 5100c34
// File: src/arc.rs


impl_arc_all!(RefMut, None);
 
// This is safe because there can only be one RefMut
unsafe impl<T: Send, M: Send + BitMask<Num=usize>> Sync for RefMut<T, M> {}

 
 /// An Rc reference which has immutable access to the inner value.
 ///
     fn clone(&self) -> Self { self.0.get_ref().unwrap() }
 }
 

// This requires T: Sync because there can be many Refs doing deref in parallel
unsafe impl<T: Send + Sync, M: Send + BitMask<Num=usize>> Sync for Ref<T, M> {}

 /// A strong reference without access to the inner value.
 ///
 /// To get immutable/mutable access, you need to use the
     fn clone(&self) -> Self { self.0.get_strong().unwrap() }
 }
 
// This is safe because there is no way to access the interior
unsafe impl<T: Send, M: Send + BitMask<Num=usize>> Sync for Strong<T, M> {}


 /// A weak reference without access to the inner value.
 ///
 /// To get immutable/mutable access, you need to use the
     impl_ref_all!();
 }
 
// This is safe because there is no way to access the interior
unsafe impl<T: Send, M: Send + BitMask<Num=usize>> Sync for Weak<T, M> {}
 
 
 #[test]