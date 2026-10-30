// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0012 (FIXED)
// Crate: backoff
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ihrwein/backoff
// Commit: 45d9248
// File: src/exponential.rs


}
 
     fn next_backoff(&mut self) -> Option<Duration> {
        let elapsed_time = self.get_elapsed_time();

         match self.max_elapsed_time {
            Some(v) if elapsed_time > v => None,
             _ => {
                 let random = rand::random::<f64>();
                 let randomized_interval = Self::get_random_value_from_interval(
                     self.current_interval,
                 );
                 self.current_interval = self.increment_current_interval();

                if let Some(max_elapsed_time) = self.max_elapsed_time {
                    if elapsed_time + randomized_interval <= max_elapsed_time {
                        Some(randomized_interval)
                    } else {
                        None
                    }
                } else {
                    Some(randomized_interval)
                }
             }
         }
     }