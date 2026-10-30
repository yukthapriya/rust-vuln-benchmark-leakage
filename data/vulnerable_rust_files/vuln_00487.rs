// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0009
// CVE: CVE-2025-4432
// CWE: N/A
// Crate: ring
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/ring
// Extraction Method: repo_search
// GitHub Repo: briansmith/ring
// Commit: 725823c
// File: src/arithmetic/bigint/boxed_limbs.rs


#[cfg(not(target_arch = "x86_64"))]
     pub(super) fn write_zeros(self) -> BoxedLimbs<M> {
        self.write_from_iter_padded(iter::empty())
             .unwrap_or_else(|LenMismatchError { .. }| unreachable!())
     }
 
         input: untrusted::Input,
     ) -> Result<BoxedLimbs<M>, LenMismatchError> {
         let input = limb::limbs_from_big_endian(input, 1..=self.len())?;
        self.write_from_iter_padded(input)
     }
 
     pub(super) fn write_copy_of_slice_checked(
         self,
         src: &[Limb],
     ) -> Result<BoxedLimbs<M>, LenMismatchError> {
        self.write_from_iter_padded(src.iter().copied())
     }
 
     pub(super) fn write_iter_checked(
         Ok(BoxedLimbs { limbs, m: self.m })
     }
 
    pub(super) fn write_from_iter_padded(
         self,
         input: impl ExactSizeIterator<Item = Limb>,
     ) -> Result<BoxedLimbs<M>, LenMismatchError>