// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0025
// CVE: N/A
// CWE: N/A
// Crate: rustc-serialize
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rustc-serialize
// Extraction Method: repo_search
// GitHub Repo: rust-lang-deprecated/rustc-serialize
// Commit: 4896e51
// File: src/serialize.rs


where T: ToOwned, T::Owned: Decodable
 {
     #[inline]
    fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'static, T>, D::Error> {
         Ok(Cow::Owned(try!(Decodable::decode(d))))
     }
 }