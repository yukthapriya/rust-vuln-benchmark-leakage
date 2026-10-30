// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0395 (FIXED)
// Crate: chrono-english
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: stevedonovan/chrono-english
// Commit: bc1b88f
// File: src/parser.rs


}
 
     fn am_pm(name: &str, mut hour: u32) -> DateResult<u32> {
        match name {
            "am" if hour == 12 => Ok(0),
            "am" => Ok(hour),
            "pm" if hour == 12 => Ok(12),
            "pm" => Ok(hour + 12),
            _ => date_result("expected am or pm"),
         }
     }
 
     fn hour_time(name: &str, hour: u32) -> DateResult<TimeSpec> {