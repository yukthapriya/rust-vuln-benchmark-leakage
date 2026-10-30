// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0112 (FIXED)
// Crate: tectonic_xdv
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tectonic-typesetting/tectonic
// Commit: 09b1c14
// File: tests/bundle_env_overrides.rs


fn test_bundle_prefix() {
     let url_prefixed = run_test_program(
         PRE_FORMAT_VERSION,
        &[
            ("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX),
            ("TECTONIC_BUNDLE_LOCKED", ""),
            // ^ locked url should be ignored if empty
        ],
     );
     let url_prefixed_versioned = run_test_program(
         TEST_FORMAT_VERSION,
        &[
            ("TECTONIC_BUNDLE_PREFIX", TEST_BUNDLE_PREFIX),
            ("TECTONIC_BUNDLE_LOCKED", ""),
        ],
     );
 
     assert_eq!(
     );
     assert_eq!(url_both_env_set, TEST_BUNDLE_LOCKED);
 }