// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0017
// CVE: N/A
// CWE: N/A
// Crate: trust-dns-proto
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/trust-dns-proto
// Extraction Method: repo_search
// GitHub Repo: hickory-dns/hickory-dns
// Commit: 157f760
// File: crates/resolver/src/name_server_pool.rs


fs::{self, File, OpenOptions},
         io::{self, Write},
         marker::PhantomData,
        path::PathBuf,
     };
 
     use tracing::trace;
                 )
             })?;
 
            if let Some(parent) = self.path.parent() {
                 fs::create_dir_all(parent)?;
             }
 
                 temp_file.sync_all()?;
             }
 
            if let Some(parent) = self.path.parent() {
                 File::open(parent)?.sync_all()?;
             }
 
             Ok(())
         }
     }
 }
 
 #[cfg(test)]