// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2018-0019
// CVE: CVE-2018-25024
// CWE: N/A
// Crate: actix-web
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/actix-web
// Extraction Method: repo_search
// GitHub Repo: actix/actix-web
// Commit: 0168411
// File: awc/src/client/h2proto.rs


let res = poll_fn(|cx| io.poll_ready(cx)).await;
     if let Err(err) = res {
        io.on_release(err.is_io());
         return Err(SendRequestError::from(err));
     }
 
             fut.await.map_err(SendRequestError::from)?
         }
         Err(err) => {
            io.on_release(err.is_io());
             return Err(err.into());
         }
     };