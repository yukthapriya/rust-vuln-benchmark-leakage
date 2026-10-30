// REAL FIXED CODE
// RustSec ID: RUSTSEC-2017-0004 (FIXED)
// Crate: base64
// Type: overflow (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: alicemaz/rust-base64
// Commit: 24ead98
// File: src/lib.rs


///}
 ///```
 pub fn encode_config<T: ?Sized + AsRef<[u8]>>(input: &T, config: Config) -> String {
    let mut buf = match encoded_size(input.as_ref().len(), config) {
        Some(n) => String::with_capacity(n),
        None => panic!("integer overflow when calculating buffer size")
    };
 
     encode_config_buf(input, config, &mut buf);
 
     buf
 }
 
 /// calculate the base64 encoded string size, including padding
fn encoded_size(bytes_len: usize, config: Config) -> Option<usize> {
    let printing_output_chars = bytes_len
        .checked_add(2)
        .map(|x| x / 3)
        .and_then(|x| x.checked_mul(4));

    //TODO this is subtly wrong but in a not dangerous way
    //pushing patch with identical to previous behavior, then fixing
     let line_ending_output_chars = match config.line_wrap {
        LineWrap::NoWrap => Some(0),
        LineWrap::Wrap(n, LineEnding::CRLF) =>
            printing_output_chars.map(|y| y / n).and_then(|y| y.checked_mul(2)),
        LineWrap::Wrap(n, LineEnding::LF) =>
            printing_output_chars.map(|y| y / n),
     };
 
    printing_output_chars.and_then(|x|
        line_ending_output_chars.and_then(|y| x.checked_add(y))
    )
 }
 
 ///Encode arbitrary octets as base64.
     };
 
     // reserve to make sure the memory we'll be writing to with unsafe is allocated
    let resv_size = match encoded_size(input_bytes.len(), config) {
        Some(n) => n,
        None => panic!("integer overflow when calculating buffer size"),
    };
    buf.reserve(resv_size);
 
     let orig_buf_len = buf.len();
     let mut fast_loop_output_buf_len = orig_buf_len;
 
     #[test]
     fn encoded_size_correct() {
        assert_eq!(Some(0), encoded_size(0, STANDARD));
 
        assert_eq!(Some(4), encoded_size(1, STANDARD));
        assert_eq!(Some(4), encoded_size(2, STANDARD));
        assert_eq!(Some(4), encoded_size(3, STANDARD));
 
        assert_eq!(Some(8), encoded_size(4, STANDARD));
        assert_eq!(Some(8), encoded_size(5, STANDARD));
        assert_eq!(Some(8), encoded_size(6, STANDARD));
 
        assert_eq!(Some(12), encoded_size(7, STANDARD));
        assert_eq!(Some(12), encoded_size(8, STANDARD));
        assert_eq!(Some(12), encoded_size(9, STANDARD));
 
        assert_eq!(Some(72), encoded_size(54, STANDARD));
 
        assert_eq!(Some(76), encoded_size(55, STANDARD));
        assert_eq!(Some(76), encoded_size(56, STANDARD));
        assert_eq!(Some(76), encoded_size(57, STANDARD));
 
        assert_eq!(Some(80), encoded_size(58, STANDARD));
     }
 
     #[test]
     fn encoded_size_correct_mime() {
        assert_eq!(Some(0), encoded_size(0, MIME));
 
        assert_eq!(Some(4), encoded_size(1, MIME));
        assert_eq!(Some(4), encoded_size(2, MIME));
        assert_eq!(Some(4), encoded_size(3, MIME));
 
        assert_eq!(Some(8), encoded_size(4, MIME));
        assert_eq!(Some(8), encoded_size(5, MIME));
        assert_eq!(Some(8), encoded_size(6, MIME));
 
        assert_eq!(Some(12), encoded_size(7, MIME));
        assert_eq!(Some(12), encoded_size(8, MIME));
        assert_eq!(Some(12), encoded_size(9, MIME));
 
        assert_eq!(Some(72), encoded_size(54, MIME));
 
        assert_eq!(Some(78), encoded_size(55, MIME));
        assert_eq!(Some(78), encoded_size(56, MIME));
        assert_eq!(Some(78), encoded_size(57, MIME));
 
        assert_eq!(Some(82), encoded_size(58, MIME));
     }
 
     #[test]
             LineWrap::Wrap(76, LineEnding::LF)
         );
 
        assert_eq!(Some(0), encoded_size(0, config));

        assert_eq!(Some(4), encoded_size(1, config));
        assert_eq!(Some(4), encoded_size(2, config));
        assert_eq!(Some(4), encoded_size(3, config));
 
        assert_eq!(Some(8), encoded_size(4, config));
        assert_eq!(Some(8), encoded_size(5, config));
        assert_eq!(Some(8), encoded_size(6, config));
 
        assert_eq!(Some(12), encoded_size(7, config));
        assert_eq!(Some(12), encoded_size(8, config));
        assert_eq!(Some(12), encoded_size(9, config));
 
        assert_eq!(Some(72), encoded_size(54, config));
 
        assert_eq!(Some(77), encoded_size(55, config));
        assert_eq!(Some(77), encoded_size(56, config));
        assert_eq!(Some(77), encoded_size(57, config));
 
        assert_eq!(Some(81), encoded_size(58, config));
    }
 
    #[test]
    fn encoded_size_overflow() {
        assert_eq!(None, encoded_size(std::usize::MAX, STANDARD));
     }
 }