// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0025 (FIXED)
// Crate: rustc-serialize
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-lang-deprecated/rustc-serialize
// Commit: 4896e51
// File: src/serialize.rs


where T: ToOwned, T::Owned: Decodable
 {
     #[inline]
    fn decode<D: Decoder>(d: &mut D) -> Result<Cow<'a, T>, D::Error> {
         Ok(Cow::Owned(try!(Decodable::decode(d))))
     }
 }