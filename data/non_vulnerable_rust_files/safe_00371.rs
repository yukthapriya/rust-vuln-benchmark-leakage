// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0076 (FIXED)
// Crate: cpython
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: dgrunwald/rust-cpython
// Commit: 54b41dc
// File: src/py_class/py_class_impl2.rs


$type_slots
             /* as_number */ [
                 $( $nb_slot_name : $nb_slot_value, )*
                nb_nonzero: $crate::py_class_unary_slot!($class::__bool__, $crate::_detail::libc::c_int, $crate::py_class::slots::BoolConverter),
             ]
             $as_sequence $as_mapping $setdelitem
         }
     { { def __lt__ $($tail:tt)* } $( $stuff:tt )* } => {
         $crate::py_error! { "__lt__ is not supported by py_class! use __richcmp__ instead." }
     };
 
     { { def __matmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "__matmul__ is not supported by py_class! yet." }
     };
     { { def __mod__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
         $class:ident $py:ident $info:tt