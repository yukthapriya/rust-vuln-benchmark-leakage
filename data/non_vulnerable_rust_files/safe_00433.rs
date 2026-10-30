// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0369 (FIXED)
// Crate: phonenumber
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: whisperfish/rust-phonenumber
// Commit: 1d11bec
// File: src/formatter.rs


use crate::formatter::Mode;
     use crate::parser;
 
    #[test]
    fn fr_leading_zero() {
        assert_eq!(
            "06 31 96 65 43",
            parser::parse(Some(country::FR), "+330631966543")
                .unwrap()
                .format()
                .mode(Mode::National)
                .to_string()
        );

        assert_eq!(
            "+33 6 31 96 65 43",
            parser::parse(Some(country::FR), "+330631966543")
                .unwrap()
                .format()
                .mode(Mode::International)
                .to_string()
        );

        assert_eq!(
            "+33631966543",
            parser::parse(Some(country::FR), "+330631966543")
                .unwrap()
                .format()
                .mode(Mode::E164)
                .to_string()
        );

        assert_eq!(
            "tel:+33-6-31-96-65-43",
            parser::parse(Some(country::FR), "+330631966543")
                .unwrap()
                .format()
                .mode(Mode::Rfc3966)
                .to_string()
        );
    }

     #[test]
     fn us() {
         assert_eq!(