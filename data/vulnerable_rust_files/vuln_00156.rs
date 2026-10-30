// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0154
// CVE: CVE-2020-36512
// CWE: N/A
// Crate: buffoon
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/buffoon
// Extraction Method: repo_search
// GitHub Repo: carllerche/buffoon
// Commit: 93c022e
// File: src/output_writer.rs


trace!("  -> size={:?}; curr={:?}", size, self.curr);
 
        if size > 0 {
            try!(write_head(self, field, WireType::LengthDelimited));
            try!(self.write_raw_varint(size));
 
             try!(val.serialize(self));
        };
 
         Ok(())
     }