// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0076
// CVE: N/A
// CWE: N/A
// Crate: cpython
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cpython
// Extraction Method: repo_search
// GitHub Repo: dgrunwald/rust-cpython
// Commit: 54b41dc
// File: src/py_class/py_class_impl2.rs


$type_slots
             /* as_number */ [
                 $( $nb_slot_name : $nb_slot_value, )*
                nb_bool: $crate::py_class_unary_slot!($class::__bool__, $crate::_detail::libc::c_int, $crate::py_class::slots::BoolConverter),
             ]
             $as_sequence $as_mapping $setdelitem
         }
     { { def __lt__ $($tail:tt)* } $( $stuff:tt )* } => {
         $crate::py_error! { "__lt__ is not supported by py_class! use __richcmp__ instead." }
     };
    { { def __matmul__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
        $class:ident $py:ident $info:tt
        /* slots: */ {
            $type_slots:tt
            /* as_number */ [ $( $nb_slot_name:ident : $nb_slot_value:expr, )* ]
            $as_sequence:tt $as_mapping:tt $setdelitem:tt
        }
        { $( $imp:item )* }
        $members:tt $props:tt
    } => { $crate::py_class_impl! {
        { $($tail)* }
        $class $py $info
        /* slots: */ {
            $type_slots
            /* as_number */ [
                $( $nb_slot_name : $nb_slot_value, )*
                nb_matrix_multiply: $crate::py_class_numeric_slot!(binary $class::__matmul__),
            ]
            $as_sequence $as_mapping $setdelitem
        }
        /* impl: */ {
            $($imp)*
            $crate::py_class_impl_item! { $class, $py, pub, __matmul__() $res_type; { $($body)* } [ { $left : &$crate::PyObject = {} } { $right : &$crate::PyObject = {} } ] }
        }
        $members $props
    }};
 
     { { def __matmul__ $($tail:tt)* } $( $stuff:tt )* } => {
        $crate::py_error! { "Invalid signature for binary numeric operator __matmul__" }
     };
     { { def __mod__($left:ident, $right:ident) -> $res_type:ty { $($body:tt)* } $($tail:tt)* }
         $class:ident $py:ident $info:tt