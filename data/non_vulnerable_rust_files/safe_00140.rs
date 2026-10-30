// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0131 (FIXED)
// Crate: rcu_cell
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Xudong-Huang/rcu_cell
// Commit: c13ca3a
// File: src/rcu_cell.rs


/// read inner ptr and check if it is the same as the given Arc
     #[inline]
     pub fn arc_eq(&self, data: &Arc<T>) -> bool {
        core::ptr::eq(self.link.get_ref(), Arc::as_ptr(data))
     }
 
     /// check if two RcuCell instances point to the same inner Arc
     #[inline]
     pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        core::ptr::eq(this.link.get_ref(), other.link.get_ref())
     }
 }