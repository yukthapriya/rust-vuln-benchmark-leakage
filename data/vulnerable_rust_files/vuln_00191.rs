// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0043
// CVE: CVE-2021-29934
// CWE: N/A
// Crate: uu_od
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/uu_od
// Extraction Method: repo_search
// GitHub Repo: uutils/coreutils
// Commit: bbec8fe
// File: src/uucore/src/lib/features/checksum/validate.rs


// For the full copyright and license information, please view the LICENSE
 // file that was distributed with this source code.
 
// spell-checker:ignore rsplit hexdigit bitlen bytelen invalidchecksum xffname
 
 use std::borrow::Cow;
 use std::ffi::OsStr;
             SubCase::OpenSSL => ByteSliceExt::rsplit_once(after_paren, b")= ")?,
         };
 
        fn is_valid_checksum(checksum: &[u8]) -> bool {
            if checksum.is_empty() {
                return false;
            }

            let mut parts = checksum.splitn(2, |&b| b == b'=');
            let main = parts.next().unwrap(); // Always exists since checksum isn't empty
            let padding = parts.next().unwrap_or_default(); // Empty if no '='

            main.iter()
                .all(|&b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/')
                && !main.is_empty()
                && padding.len() <= 2
                && padding.iter().all(|&b| b == b'=')
        }
        if !is_valid_checksum(checksum) {
            return None;
        }
        // SAFETY: we just validated the contents of checksum, we can unsafely make a
        // String from it
        let checksum_utf8 = unsafe { String::from_utf8_unchecked(checksum.to_vec()) };
 
         Some(LineInfo {
             algo_name: Some(algo_utf8),
     fn parse_untagged(line: &[u8]) -> Option<LineInfo> {
         let space_idx = line.iter().position(|&b| b == b' ')?;
         let checksum = &line[..space_idx];
        if !checksum.iter().all(|&b| b.is_ascii_hexdigit()) || checksum.is_empty() {
            return None;
        }
        // SAFETY: we just validated the contents of checksum, we can unsafely make a
        // String from it
        let checksum_utf8 = unsafe { String::from_utf8_unchecked(checksum.to_vec()) };
 
         let rest = &line[space_idx..];
         let filename = rest
             format: Self::SingleSpace,
         })
     }
 }
 
 // Helper trait for byte slice operations
                 b"b064a020db8018f18ff5ae367d01b212   ",
                 Some((b"b064a020db8018f18ff5ae367d01b212", b" ")),
             ),
            (b"invalidchecksum  test", None),
         ];
 
         for (input, expected) in test_cases {