// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0395
// CVE: N/A
// CWE: N/A
// Crate: chrono-english
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/chrono-english
// Extraction Method: repo_search
// GitHub Repo: stevedonovan/chrono-english
// Commit: bc1b88f
// File: src/parser.rs


}
 
     fn am_pm(name: &str, mut hour: u32) -> DateResult<u32> {
        if name == "pm" {
            hour += 12;
        } else if name != "am" {
            return date_result("expected am or pm");
         }
        Ok(hour)
     }
 
     fn hour_time(name: &str, hour: u32) -> DateResult<TimeSpec> {