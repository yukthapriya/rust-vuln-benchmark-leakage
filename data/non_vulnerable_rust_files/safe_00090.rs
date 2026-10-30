// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0061 (FIXED)
// Crate: futures-task
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang/futures-rs
// Commit: 3116fda
// File: futures-util/src/future/future/remote_handle.rs


}
 }
 
type SendMsg<Fut> = Result<<Fut as Future>::Output, Box<dyn Any + Send + 'static>>;
 
 pin_project! {
     /// A future which sends its output to the corresponding `RemoteHandle`.