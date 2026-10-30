// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0139 (FIXED)
// Crate: ansi_term
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ogham/rust-ansi-term
// Commit: e2801d3
// File: src/ansi.rs


Prefix(self)
     }
 
    /// The infix bytes between this style and `next` style. These are the bytes
    /// that tell the terminal to change the style to `next`. These may include
    /// a reset followed by the next colour and style, depending on the two styles.
     ///
     /// # Examples
     ///
     /// assert_eq!("",
     ///            style.infix(style).to_string());
     /// ```
    pub fn infix(self, next: Style) -> Infix {
        Infix(self, next)
     }
 
     /// The suffix for this style. These are the bytes that tell the terminal
         Prefix(self.normal())
     }
 
    /// The infix bytes between this colour and `next` colour. These are the bytes
    /// that tell the terminal to use the `next` colour, or to do nothing if
     /// the two colours are equal.
     ///
     /// See also [`Style::infix`](struct.Style.html#method.infix).
     /// assert_eq!("\x1b[33m",
     ///            Red.infix(Yellow).to_string());
     /// ```
    pub fn infix(self, next: Colour) -> Infix {
        Infix(self.normal(), next.normal())
     }
 
     /// The suffix for this colour as a `Style`. These are the bytes that
             },
             Difference::Reset => {
                 let f: &mut fmt::Write = f;
                write!(f, "{}{}", RESET, self.1.prefix())
             },
             Difference::NoDifference => {
                 Ok(())   // nothing to write
     test!(hidden:                Style::new().hidden();             "hi" => "\x1B[8mhi\x1B[0m");
     test!(stricken:              Style::new().strikethrough();      "hi" => "\x1B[9mhi\x1B[0m");
 
    #[test]
    fn test_infix() {
        assert_eq!(Style::new().dimmed().infix(Style::new()).to_string(), "\x1B[0m");
        assert_eq!(White.dimmed().infix(White.normal()).to_string(), "\x1B[0m\x1B[37m");
        assert_eq!(White.normal().infix(White.bold()).to_string(), "\x1B[1m");
        assert_eq!(White.normal().infix(Blue.normal()).to_string(), "\x1B[34m");
        assert_eq!(Blue.bold().infix(Blue.bold()).to_string(), "");
    }
 }