// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0121 (FIXED)
// Crate: abox
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: SonicFrog/abox
// Commit: e6fb03b
// File: src/lib.rs


}
 }
 
unsafe impl<T: Sized + Sync> Sync for AtomicBox<T> {}
unsafe impl<T: Sized + Send> Send for AtomicBox<T> {}
 
 #[cfg(test)]
 mod tests {