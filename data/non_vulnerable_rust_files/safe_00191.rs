// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0043 (FIXED)
// Crate: uu_od
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: uutils/coreutils
// Commit: bbec8fe
// File: src/uucore/src/lib/features/checksum/validate.rs


// For the full copyright and license information, please view the LICENSE
 // file that was distributed with this source code.
 
// spell-checker:ignore rsplit hexdigit bitlen bytelen invalidchecksum inva idchecksum xffname
 
 use std::borrow::Cow;
 use std::ffi::OsStr;
             SubCase::OpenSSL => ByteSliceExt::rsplit_once(after_paren, b")= ")?,
         };
 
        let checksum_utf8 = Self::validate_checksum_format(checksum)?;
 
         Some(LineInfo {
             algo_name: Some(algo_utf8),
     fn parse_untagged(line: &[u8]) -> Option<LineInfo> {
         let space_idx = line.iter().position(|&b| b == b' ')?;
         let checksum = &line[..space_idx];

        let checksum_utf8 = Self::validate_checksum_format(checksum)?;
 
         let rest = &line[space_idx..];
         let filename = rest
             format: Self::SingleSpace,
         })
     }

    /// Ensure that the given checksum is syntactically valid (that it is either
    /// hexadecimal or base64 encoded).
    fn validate_checksum_format(checksum: &[u8]) -> Option<String> {
        if checksum.is_empty() {
            return None;
        }

        let mut parts = checksum.splitn(2, |&b| b == b'=');
        let main = parts.next().unwrap(); // Always exists since checksum isn't empty
        let padding = parts.next().unwrap_or_default(); // Empty if no '='

        if main.is_empty()
            || !main
                .iter()
                .all(|&b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/')
        {
            return None;
        }

        if padding.len() > 2 || padding.iter().any(|&b| b != b'=') {
            return None;
        }

        // SAFETY: we just validated the contents of checksum, we can unsafely make a
        // String from it
        Some(unsafe { String::from_utf8_unchecked(checksum.to_vec()) })
    }
 }
 
 // Helper trait for byte slice operations
                 b"b064a020db8018f18ff5ae367d01b212   ",
                 Some((b"b064a020db8018f18ff5ae367d01b212", b" ")),
             ),
            // base64 checksums are accepted
            (
                b"b21lbGV0dGUgZHUgZnJvbWFnZQ==   ",
                Some((b"b21lbGV0dGUgZHUgZnJvbWFnZQ==", b" ")),
            ),
            // Invalid checksums fail
            (b"inva|idchecksum  test", None),
         ];
 
         for (input, expected) in test_cases {