// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0029 (FIXED)
// Crate: truetype
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: bodoni/truetype
// Commit: 1d50ea8
// File: src/tables/names/encoding/macintosh.rs


}
     };
     for character in value.chars() {
        if character as u16 <= 0x7F {
            data.push(character as u8);
        } else if let Some(value) = mapping.get(&character) {
            data.push(*value | 0x80);
         } else {
             raise!("found an unknown Macintosh character ({character})")
         }