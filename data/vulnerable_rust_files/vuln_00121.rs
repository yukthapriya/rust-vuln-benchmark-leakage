// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0108
// CVE: CVE-2020-36216
// CWE: N/A
// Crate: eventio
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/eventio
// Extraction Method: repo_search
// GitHub Repo: petabi/eventio
// Commit: c5428ed
// File: src/kafka.rs


impl crate::Event for Event {
     type Ack = EntryLocation;
 
    #[must_use]
     fn raw(&self) -> &[u8] {
         self.entry
             .record
             .get("message")
             .map_or(b"", |v| v.as_slice())
     }
 
    #[must_use]
     fn time(&self) -> super::SeqNo {
         super::SeqNo::from_ne_bytes(self.entry.time.to_ne_bytes())
     }
 
    #[must_use]
     fn ack(&self) -> Self::Ack {
         self.loc
     }