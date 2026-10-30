// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0007 (FIXED)
// Crate: av-data
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-av/rust-av
// Commit: 71680cf
// File: bitstream/src/bytewrite.rs


$(
                 paste::item! {
                     #[test]
                    #[allow(clippy::float_equality_without_abs)]
                     fn [<put_and_get_ $TYPE _be>]() {
                         let item_size = size_of::<$TYPE>();
                         let mut buf = vec![0; 3 * item_size];
                         [<put_ $TYPE b>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE - [<get_ $TYPE b>](&buf) < $TYPE::EPSILON);
 
                         [<put_ $TYPE b>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE - [<get_ $TYPE b>](&buf[(1 * item_size)..]) < $TYPE::EPSILON);
 
                         [<put_ $TYPE b>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE - [<get_ $TYPE b>](&buf[(2 * item_size)..]) < $TYPE::EPSILON);
                     }
                 }
 
                 paste::item! {
                     #[test]
                    #[allow(clippy::float_equality_without_abs)]
                     fn [<put_and_get_ $TYPE _l>]() {
                         let item_size = size_of::<$TYPE>();
                         let mut buf = vec![0; 3 * item_size];
                         [<put_ $TYPE l>](&mut buf, 1 as $TYPE);
                        assert!(1 as $TYPE - [<get_ $TYPE l>](&buf) < $TYPE::EPSILON);
 
                         [<put_ $TYPE l>](&mut buf[(1 * item_size)..], 2 as $TYPE);
                        assert!(2 as $TYPE - [<get_ $TYPE l>](&buf[(1 * item_size)..]) < $TYPE::EPSILON);
 
                         [<put_ $TYPE l>](&mut buf[(2 * item_size)..], 255 as $TYPE);
                        assert!(255 as $TYPE - [<get_ $TYPE l>](&buf[(2 * item_size)..]) < $TYPE::EPSILON);
                     }
                 }
             )*