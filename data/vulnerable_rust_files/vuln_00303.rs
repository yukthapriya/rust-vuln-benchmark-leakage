// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0072
// CVE: N/A
// CWE: N/A
// Crate: hyper-staticfile
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hyper-staticfile
// Extraction Method: commit_diff
// GitHub Repo: stephank/hyper-staticfile
// Commit: f12cadc
// File: src/resolve.rs


/// The requested file could not be accessed.
     PermissionDenied,
     /// A directory was requested as a file.
    IsDirectory,
     /// The requested file was found.
     Found(ResolvedFile<F>),
 }
 
         // We may have opened a directory for a file request, in which case we redirect.
         if !is_dir_request && file.is_dir {
            return Ok(ResolveResult::IsDirectory);
         }
 
         // If not a directory, serve this file.