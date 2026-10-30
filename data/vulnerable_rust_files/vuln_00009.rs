// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2017-0004
// CVE: CVE-2017-1000430
// CWE: N/A
// Crate: base64
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/base64
// Extraction Method: commit_diff
// GitHub Repo: alicemaz/rust-base64
// Commit: 24ead98
// File: src/lib.rs


///}
 ///```
 pub fn encode_config<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config) -> String {
    let mut buf = String::with_capacity(encoded_size(input.as_ref().len(), config));
 
     encode_config_buf(input, config, &mut buf);
 
     buf
 }
 
 /// calculate the base64 encoded string size, including padding
fn encoded_size(bytes_len: usize, config: Config) -> usize {
    let rem = bytes_len % 3;

    let complete_input_chunks = bytes_len / 3;
    let complete_output_chars = complete_input_chunks * 4;
    let printing_output_chars = if rem == 0 {
        complete_output_chars
    } else {
        complete_output_chars + 4
    };
     let line_ending_output_chars = match config.line_wrap {
        LineWrap::NoWrap => 0,
        LineWrap::Wrap(n, LineEnding::CRLF) => printing_output_chars / n * 2,
        LineWrap::Wrap(n, LineEnding::LF) => printing_output_chars / n,
     };
 
    return printing_output_chars + line_ending_output_chars;
 }
 
 ///Encode arbitrary octets as base64.
     };
 
     // reserve to make sure the memory we'll be writing to with unsafe is allocated
    buf.reserve(encoded_size(input_bytes.len(), config));
 
     let orig_buf_len = buf.len();
     let mut fast_loop_output_buf_len = orig_buf_len;
 
     #[test]
     fn encoded_size_correct() {
        assert_eq!(0, encoded_size(0, STANDARD));
 
        assert_eq!(4, encoded_size(1, STANDARD));
        assert_eq!(4, encoded_size(2, STANDARD));
        assert_eq!(4, encoded_size(3, STANDARD));
 
        assert_eq!(8, encoded_size(4, STANDARD));
        assert_eq!(8, encoded_size(5, STANDARD));
        assert_eq!(8, encoded_size(6, STANDARD));
 
        assert_eq!(12, encoded_size(7, STANDARD));
        assert_eq!(12, encoded_size(8, STANDARD));
        assert_eq!(12, encoded_size(9, STANDARD));
 
        assert_eq!(72, encoded_size(54, STANDARD));
 
        assert_eq!(76, encoded_size(55, STANDARD));
        assert_eq!(76, encoded_size(56, STANDARD));
        assert_eq!(76, encoded_size(57, STANDARD));
 
        assert_eq!(80, encoded_size(58, STANDARD));
     }
 
     #[test]
     fn encoded_size_correct_mime() {
        assert_eq!(0, encoded_size(0, MIME));
 
        assert_eq!(4, encoded_size(1, MIME));
        assert_eq!(4, encoded_size(2, MIME));
        assert_eq!(4, encoded_size(3, MIME));
 
        assert_eq!(8, encoded_size(4, MIME));
        assert_eq!(8, encoded_size(5, MIME));
        assert_eq!(8, encoded_size(6, MIME));
 
        assert_eq!(12, encoded_size(7, MIME));
        assert_eq!(12, encoded_size(8, MIME));
        assert_eq!(12, encoded_size(9, MIME));
 
        assert_eq!(72, encoded_size(54, MIME));
 
        assert_eq!(78, encoded_size(55, MIME));
        assert_eq!(78, encoded_size(56, MIME));
        assert_eq!(78, encoded_size(57, MIME));
 
        assert_eq!(82, encoded_size(58, MIME));
     }
 
     #[test]
             LineWrap::Wrap(76, LineEnding::LF)
         );
 
        assert_eq!(0, encoded_size(0, config));
 
        assert_eq!(4, encoded_size(1, config));
        assert_eq!(4, encoded_size(2, config));
        assert_eq!(4, encoded_size(3, config));
 
        assert_eq!(8, encoded_size(4, config));
        assert_eq!(8, encoded_size(5, config));
        assert_eq!(8, encoded_size(6, config));
 
        assert_eq!(12, encoded_size(7, config));
        assert_eq!(12, encoded_size(8, config));
        assert_eq!(12, encoded_size(9, config));
 
        assert_eq!(72, encoded_size(54, config));
 
        assert_eq!(77, encoded_size(55, config));
        assert_eq!(77, encoded_size(56, config));
        assert_eq!(77, encoded_size(57, config));
 
        assert_eq!(81, encoded_size(58, config));
     }
 }