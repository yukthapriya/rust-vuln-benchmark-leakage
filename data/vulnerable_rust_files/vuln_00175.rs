// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0017
// CVE: CVE-2021-26953
// CWE: N/A
// Crate: postscript
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/postscript
// Extraction Method: repo_search
// GitHub Repo: bodoni/postscript
// Commit: 29a304b
// File: src/type2/number.rs


use crate::Result;
 
const FIXED_SCALING: f32 = 1f32 / (1 << 16) as f32;
 
 pub fn read<T: crate::tape::Read>(tape: &mut T) -> Result<f32> {
     let first = tape.take::<u8>()?;
         0xf7..=0xfa => ((first as i32 - 247) * 256 + tape.take::<u8>()? as i32 + 108) as f32,
         0xfb..=0xfe => (-(first as i32 - 251) * 256 - tape.take::<u8>()? as i32 - 108) as f32,
         0x1c => tape.take::<u16>()? as i16 as i32 as f32,
        0xff => FIXED_SCALING * (tape.take::<u32>()? as f32),
         _ => raise!("found a malformed number"),
     })
 }