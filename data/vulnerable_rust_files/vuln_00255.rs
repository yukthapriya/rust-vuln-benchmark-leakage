// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0139
// CVE: N/A
// CWE: N/A
// Crate: ansi_term
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ansi_term
// Extraction Method: repo_search
// GitHub Repo: ogham/rust-ansi-term
// Commit: e2801d3
// File: src/ansi.rs


Prefix(self)
     }
 
    /// The infix bytes between this style and another. These are the bytes
    /// that tell the terminal to either use a different colour or font style
    /// _or_ to reset entirely, depending on the two styles.
     ///
     /// # Examples
     ///
     /// assert_eq!("",
     ///            style.infix(style).to_string());
     /// ```
    pub fn infix(self, other: Style) -> Infix {
        Infix(self, other)
     }
 
     /// The suffix for this style. These are the bytes that tell the terminal
         Prefix(self.normal())
     }
 
    /// The infix bytes between this style and another. These are the bytes
    /// that tell the terminal to use the second colour, or to do nothing if
     /// the two colours are equal.
     ///
     /// See also [`Style::infix`](struct.Style.html#method.infix).
     /// assert_eq!("\x1b[33m",
     ///            Red.infix(Yellow).to_string());
     /// ```
    pub fn infix(self, other: Colour) -> Infix {
        Infix(self.normal(), other.normal())
     }
 
     /// The suffix for this colour as a `Style`. These are the bytes that
             },
             Difference::Reset => {
                 let f: &mut fmt::Write = f;
                write!(f, "{}{}", RESET, self.0.prefix())
             },
             Difference::NoDifference => {
                 Ok(())   // nothing to write
     test!(hidden:                Style::new().hidden();             "hi" => "\x1B[8mhi\x1B[0m");
     test!(stricken:              Style::new().strikethrough();      "hi" => "\x1B[9mhi\x1B[0m");
 
 }