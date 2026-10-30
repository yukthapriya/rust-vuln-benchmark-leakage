// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0121
// CVE: CVE-2020-36441
// CWE: N/A
// Crate: abox
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/abox
// Extraction Method: repo_search
// GitHub Repo: SonicFrog/abox
// Commit: e6fb03b
// File: src/lib.rs


}
 }
 
unsafe impl<T: Sized> Sync for AtomicBox<T> {}
unsafe impl<T: Sized> Send for AtomicBox<T> {}
 
 #[cfg(test)]
 mod tests {