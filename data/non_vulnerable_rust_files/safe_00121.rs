// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0108 (FIXED)
// Crate: eventio
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: petabi/eventio
// Commit: c5428ed
// File: src/kafka.rs


impl crate::Event for Event {
     type Ack = EntryLocation;
 
     fn raw(&self) -> &[u8] {
         self.entry
             .record
             .get("message")
             .map_or(b"", |v| v.as_slice())
     }
 
     fn time(&self) -> super::SeqNo {
         super::SeqNo::from_ne_bytes(self.entry.time.to_ne_bytes())
     }
 
     fn ack(&self) -> Self::Ack {
         self.loc
     }