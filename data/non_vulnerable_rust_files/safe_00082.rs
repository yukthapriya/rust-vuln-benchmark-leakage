// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0048 (FIXED)
// Crate: actix-http
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: actix/actix-web
// Commit: 0168411
// File: awc/src/client/h2proto.rs


let res = poll_fn(|cx| io.poll_ready(cx)).await;
     if let Err(err) = res {
        io.on_release(err.is_io() || err.is_go_away());
         return Err(SendRequestError::from(err));
     }
 
             fut.await.map_err(SendRequestError::from)?
         }
         Err(err) => {
            io.on_release(err.is_io() || err.is_go_away());
             return Err(err.into());
         }
     };