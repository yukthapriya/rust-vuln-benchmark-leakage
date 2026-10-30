// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0071
// CVE: CVE-2020-26235
// CWE: N/A
// Crate: time
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/time
// Extraction Method: repo_search
// GitHub Repo: time-rs/time
// Commit: fc3c8b2
// File: tests/quickcheck.rs


#[quickcheck]
 fn odt_to_offset_no_panic(odt: OffsetDateTime, offset: UtcOffset) -> TestResult {
     if Date::MIN
         .midnight()
         .assume_utc()
        .checked_add(Duration::seconds(offset.whole_seconds().extend()))
         .is_none()
         || Date::MAX
             .with_time(time!(23:59:59.999_999_999))
             .assume_utc()
            .checked_add(Duration::seconds(offset.whole_seconds().extend()))
             .is_none()
     {
         return TestResult::discard();