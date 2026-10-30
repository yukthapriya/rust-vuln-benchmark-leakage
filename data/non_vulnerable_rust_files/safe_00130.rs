// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0118 (FIXED)
// Crate: tiny_future
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: KizzyCode/tiny_future
// Commit: 7ab8a26
// File: src/lib.rs


/// Creates a future for `job` and runs `job`. The result of `job` will be set as result into the
 /// future. The parameter passed to `job` is a function that returns if the future is still waiting
 /// so that `job` can check for cancellation.
pub fn async_with_state<T: 'static + Send, U: 'static + Send, F: FnOnce(Future<T, U>) + Send + 'static>(job: F, shared_state: U) -> Future<T, U> {
 	use std::clone::Clone;
 	
 	// Create future and spawn job
 /// Creates a future for `job` and runs `job`. The result of `job` will be set as result into the
 /// future. The parameter passed to `job` is a function that returns if the future is still waiting
 /// so that `job` can check for cancellation.
pub fn async<T: 'static + Send, F: FnOnce(Future<T, ()>) + Send + 'static>(job: F) -> Future<T, ()> {
 	async_with_state(job, ())
 }