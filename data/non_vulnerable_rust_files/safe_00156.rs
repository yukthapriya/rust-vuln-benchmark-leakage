// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0154 (FIXED)
// Crate: buffoon
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: carllerche/buffoon
// Commit: 93c022e
// File: src/output_writer.rs


trace!("  -> size={:?}; curr={:?}", size, self.curr);
 
        try!(write_head(self, field, WireType::LengthDelimited));
        try!(self.write_raw_varint(size));
 
        if size > 0 {
             try!(val.serialize(self));
        }
 
         Ok(())
     }