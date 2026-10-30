// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0087 (FIXED)
// Crate: columnar
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: frankmcsherry/columnar
// Commit: bf2fa42
// File: columnar_derive/src/lib.rs


// type Borrowed<'columnar> = #c_ident;
             #[inline(always)]
             fn as_bytes(&self) -> impl Iterator<Item=(u64, &'a [u8])> {
                std::iter::once((8, ::columnar::bytemuck::cast_slice(std::slice::from_ref(self.count))))
             }
         }
 
         impl<'columnar> ::columnar::FromBytes<'columnar> for #c_ident <&'columnar u64> {
             #[inline(always)]
             fn from_bytes(bytes: &mut impl Iterator<Item=&'columnar [u8]>) -> Self {
                Self { count: &::columnar::bytemuck::try_cast_slice(bytes.next().unwrap()).unwrap()[0] }
             }
         }