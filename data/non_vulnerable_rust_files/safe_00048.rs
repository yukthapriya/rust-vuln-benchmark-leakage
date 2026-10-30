// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0036 (FIXED)
// Crate: failure
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang-nursery/failure
// Commit: 6a12641
// File: src/compat.rs


}
     }
 
    impl From<Error> for Box<StdError> {
        fn from(error: Error) -> Box<StdError> {
            Box::new(Compat { error })
        }
    }

     impl From<Error> for Box<StdError + Send + Sync> {
         fn from(error: Error) -> Box<StdError + Send + Sync> {
             Box::new(Compat { error })