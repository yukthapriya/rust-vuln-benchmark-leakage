// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0014
// CVE: N/A
// CWE: N/A
// Crate: humantime
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/humantime
// Extraction Method: repo_search
// GitHub Repo: tailhook/humantime
// Commit: ea2ed40
// File: src/duration.rs


_ => {
                 if self.src.chars().all(|c| c == '0') {
                     self.current = (0, 0);
                    return Ok(())
                 }
                 return Err(Error::UnknownUnit {
                     start,
         assert_eq!(parse_duration("365d"), Ok(Duration::new(31_536_000, 0)));
         assert_eq!(parse_duration("1week"), Ok(Duration::new(604_800, 0)));
         assert_eq!(parse_duration("7weeks"), Ok(Duration::new(4_233_600, 0)));
        assert_eq!(parse_duration("104wks"), Ok(Duration::new(2*31_449_600, 0)));
         assert_eq!(parse_duration("100wk"), Ok(Duration::new(60_480_000, 0)));
         assert_eq!(parse_duration("52w"), Ok(Duration::new(31_449_600, 0)));
         assert_eq!(parse_duration("1month"), Ok(Duration::new(2_630_016, 0)));
         );
         assert_eq!(
             parse_duration("15yrs"),
            Ok(Duration::new(15*31_557_600, 0))
         );
         assert_eq!(
             parse_duration("10yr"),
            Ok(Duration::new(10*31_557_600, 0))
         );
         assert_eq!(parse_duration("17y"), Ok(Duration::new(536_479_200, 0)));
     }
     #[cfg(feature = "mu")]
     #[test]
     fn test_format_micros() {
        assert_eq!(format_duration(Duration::from_micros(123)).to_string(), "123µs");
     }
 
     #[cfg(not(feature = "mu"))]
     #[test]
     fn test_format_micros() {
        assert_eq!(format_duration(Duration::from_micros(123)).to_string(), "123us");
     }
 
     #[test]