// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0012
// CVE: N/A
// CWE: N/A
// Crate: backoff
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/backoff
// Extraction Method: repo_search
// GitHub Repo: ihrwein/backoff
// Commit: 45d9248
// File: src/exponential.rs


}
 
     fn next_backoff(&mut self) -> Option<Duration> {
         match self.max_elapsed_time {
            Some(v) if self.get_elapsed_time() > v => None,
             _ => {
                 let random = rand::random::<f64>();
                 let randomized_interval = Self::get_random_value_from_interval(
                     self.current_interval,
                 );
                 self.current_interval = self.increment_current_interval();
                Some(randomized_interval)
             }
         }
     }