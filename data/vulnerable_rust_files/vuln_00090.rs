// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0061
// CVE: CVE-2020-35907
// CWE: N/A
// Crate: futures-task
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/futures-task
// Extraction Method: repo_search
// GitHub Repo: rust-lang/futures-rs
// Commit: 3116fda
// File: futures-util/src/future/future/remote_handle.rs


}
 }
 
type SendMsg<Fut> = Result<<Fut as Future>::Output, Box<(dyn Any + Send + 'static)>>;
 
 pin_project! {
     /// A future which sends its output to the corresponding `RemoteHandle`.