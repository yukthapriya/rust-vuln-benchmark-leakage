// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0015
// CVE: N/A
// CWE: N/A
// Crate: ascii
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ascii
// Extraction Method: repo_search
// GitHub Repo: tomprogrammer/rust-ascii
// Commit: 7203d9a
// File: src/ascii_char.rs


Null = 0,
     /// [Start Of Heading](http://en.wikipedia.org/wiki/Start_of_Heading)
     SOH = 1,
    /// [Start Of teXt](http://en.wikipedia.org/wiki/Start_of_Text)
     SOX = 2,
     /// [End of TeXt](http://en.wikipedia.org/wiki/End-of-Text_character)
     ETX = 3,
 }
 
 impl AsciiChar {
     /// Constructs an ASCII character from a `u8`, `char` or other character type.
     ///
     /// # Errors
 
         #[rustfmt::skip]
         const ALL: [AsciiChar; 128] = [
            Null, SOH, SOX, ETX, EOT, ENQ, ACK, Bell,
             BackSpace, Tab, LineFeed, VT, FF, CarriageReturn, SI, SO,
             DLE, DC1, DC2, DC3, DC4, NAK, SYN, ETB,
             CAN, EM, SUB, ESC, FS, GS, RS, US,