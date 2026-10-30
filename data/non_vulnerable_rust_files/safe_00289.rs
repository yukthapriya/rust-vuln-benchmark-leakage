// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0048 (FIXED)
// Crate: xml-rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: netvl/xml-rs
// Commit: e481000
// File: src/reader/parser.rs


self.into_state_continue(State::InsideReference)
             },
 
            Token::OpeningTagStart | Token::ProcessingInstructionStart => Some(self.error(SyntaxError::UnexpectedOpeningTag)),
 
             Token::Character(c) if !self.is_valid_xml_char_not_restricted(c) => {
                 Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
             }
         );
     }
 
    #[test]
    fn processing_instruction_in_attribute_value() {
        use crate::reader::error::{SyntaxError, Error, ErrorKind};

        let (mut r, mut p) = test_data!(r#"
            <y F="<?abc"><x G="/">
        "#);

        expect_event!(r, p, Ok(XmlEvent::StartDocument { .. }));
        expect_event!(r, p, Err(ref e) =>
            *e == Error {
                kind: ErrorKind::Syntax(SyntaxError::UnexpectedOpeningTag.to_cow()),
                pos: TextPosition { row: 1, column: 18 }
            }
        );
    }
 
     #[test]
     fn reference_err() {