// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0071 (FIXED)
// Crate: time
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: time-rs/time
// Commit: fc3c8b2
// File: tests/quickcheck.rs


#[quickcheck]
 fn odt_to_offset_no_panic(odt: OffsetDateTime, offset: UtcOffset) -> TestResult {
    let offset_difference = offset.whole_seconds() - odt.offset().whole_seconds();
     if Date::MIN
         .midnight()
         .assume_utc()
        .checked_add(Duration::seconds(offset_difference.extend()))
         .is_none()
         || Date::MAX
             .with_time(time!(23:59:59.999_999_999))
             .assume_utc()
            .checked_add(Duration::seconds(offset_difference.extend()))
             .is_none()
     {
         return TestResult::discard();