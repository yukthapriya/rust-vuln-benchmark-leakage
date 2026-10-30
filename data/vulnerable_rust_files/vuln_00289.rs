// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0048
// CVE: N/A
// CWE: N/A
// Crate: xml-rs
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/xml-rs
// Extraction Method: repo_search
// GitHub Repo: netvl/xml-rs
// Commit: e481000
// File: src/reader/parser.rs


self.into_state_continue(State::InsideReference)
             },
 
            Token::OpeningTagStart => Some(self.error(SyntaxError::UnexpectedOpeningTag)),
 
             Token::Character(c) if !self.is_valid_xml_char_not_restricted(c) => {
                 Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
             }
         );
     }
 
     #[test]
     fn reference_err() {