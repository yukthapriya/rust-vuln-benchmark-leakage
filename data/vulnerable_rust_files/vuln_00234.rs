// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0112
// CVE: CVE-2021-45703
// CWE: N/A
// Crate: tectonic_xdv
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tectonic_xdv
// Extraction Method: repo_search
// GitHub Repo: tectonic-typesetting/tectonic
// Commit: 09b1c14
// File: tests/bundle_env_overrides.rs


fn test_bundle_prefix() {
     let url_prefixed = run_test_program(
         PRE_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX)],
     );
     let url_prefixed_versioned = run_test_program(
         TEST_FORMAT_VERSION,
        &[("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX)],
     );
 
     assert_eq!(
     );
     assert_eq!(url_both_env_set, TEST_BUNDLE_LOCKED);
 }

#[test]
fn test_empty_locked_bundle_ignored() {
    let url_empty_locked = run_test_program(
        TEST_FORMAT_VERSION,
        &[
            ("TECTONIC_BUNDLE_LOCKED", ""),
            ("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX),
        ],
    );
    assert_eq!(
        url_empty_locked,
        format!("{TEST_BUNDLE_PREFIX}/default_bundle_v{TEST_FORMAT_VERSION}.tar",)
    );
}