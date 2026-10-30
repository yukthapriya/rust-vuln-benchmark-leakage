// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0015 (FIXED)
// Crate: ascii
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tomprogrammer/rust-ascii
// Commit: 7203d9a
// File: src/ascii_char.rs


Null = 0,
     /// [Start Of Heading](http://en.wikipedia.org/wiki/Start_of_Heading)
     SOH = 1,
    #[allow(clippy::doc_markdown)]
    /// [Start of TeXt](http://en.wikipedia.org/wiki/Start_of_Text)
    #[doc(hidden)]
    #[deprecated(since="1.2.0", note="Replaced with AsciiChar::STX which is the correct name for this variant.")]
     SOX = 2,
     /// [End of TeXt](http://en.wikipedia.org/wiki/End-of-Text_character)
     ETX = 3,
 }
 
 impl AsciiChar {
    /// [Start of TeXt](http://en.wikipedia.org/wiki/Start_of_Text)
    ///
    /// (It's an associated constant instead of a variant because
    ///  the variant for it has an incorrect name.)
    #[allow(deprecated, clippy::doc_markdown)]
    pub const STX: AsciiChar = AsciiChar::SOX;

     /// Constructs an ASCII character from a `u8`, `char` or other character type.
     ///
     /// # Errors
 
         #[rustfmt::skip]
         const ALL: [AsciiChar; 128] = [
            Null, SOH, AsciiChar::STX, ETX, EOT, ENQ, ACK, Bell,
             BackSpace, Tab, LineFeed, VT, FF, CarriageReturn, SI, SO,
             DLE, DC1, DC2, DC3, DC4, NAK, SYN, ETB,
             CAN, EM, SUB, ESC, FS, GS, RS, US,