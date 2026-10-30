// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0017
// CVE: N/A
// CWE: N/A
// Crate: maligned
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/maligned
// Extraction Method: repo_search
// GitHub Repo: tylerhawkes/maligned
// Commit: c7adf3d
// File: src/align.rs


pub trait SealedAsBytesMut: SealedAsBytes {}
 }
 
/// Marker trait used for bounds. Public so that it can be extended if necessary.
/// All structs that implement this trait in `maligned` have their size and alignment equal.
 pub trait Alignment: sealed_traits::SealedAlignment + Sized + Clone + Default + Send + Sync {}
 
 /// Trait that allows reinterpretation of a struct as bytes. This is unsafe since the
 #[allow(unsafe_code)]
 pub unsafe trait AsBytes: sealed_traits::SealedAsBytes + Sized {
   /// Turn a reference to self to a byte slice.
   fn as_bytes(&self) -> &[u8];
 }
 
 #[allow(unsafe_code)]
 pub unsafe trait AsBytesMut: sealed_traits::SealedAsBytesMut + AsBytes {
   /// Turn a mutable reference to self to a mutable byte slice.
   fn as_bytes_mut(&mut self) -> &mut [u8];
 }
 
 }
 
 impl FromByteSliceError {
   /// Returns the length of the slice that failed to convert to an alignment type.
   pub fn actual_len(&self) -> usize {
     self.actual_len
   }
   /// Returns the length that the alignment type expected the slice to be when converting.
   pub fn expected_len(&self) -> usize {
     self.expected_len
   }
 }
 
 macro_rules! impl_alignment_traits {
   {
    $(#[$outer:meta])+
     $v:vis struct $t:ident or $bit:ident $(or $k_alias:ident)?(pub [u8; $c:literal]);
     test = $test:ident
   }
     => {
    $(#[$outer])+
    #[repr(align($c))]
    $v struct $t(pub [u8; $c]);
     impl sealed_traits::SealedAlignment for $t {}
     impl Alignment for $t {}
    /// Type alias for an Alignment in bits
    pub type $bit = $t;
     $(
    /// Type alias for an Alignment in kilobytes
    pub type $k_alias = $t;
     )?
     // Not a trait since const generics aren't stable yet
     #[doc(hidden)]
     impl $t {
       pub fn as_array(&self) -> &[u8; size_of::<Self>()] {
         &self.0
       }
       pub fn as_array_mut(&mut self) -> &mut [u8; size_of::<Self>()] {
         &mut self.0
       }
       pub fn into_array(self) -> [u8; size_of::<Self>()] {
         self.0
       }
     #[allow(unsafe_code)]
     unsafe impl AsBytes for $t {
       #[inline]
       fn as_bytes(&self) -> &[u8] {
         unsafe { core::slice::from_raw_parts(&self.0 as *const [u8; size_of::<Self>()] as *const u8, size_of::<Self>()) }
       }
     #[allow(unsafe_code)]
     unsafe impl AsBytesMut for $t {
       #[inline]
       fn as_bytes_mut(&mut self) -> &mut [u8] {
         unsafe { core::slice::from_raw_parts_mut(&mut self.0 as *mut [u8; size_of::<Self>()] as *mut u8, size_of::<Self>()) }
       }
         f.debug_tuple(stringify!($t)).field(&self.as_bytes()).finish()
       }
     }
    #[test]
    fn $test() {
      use std::convert::TryFrom;
      use std::collections::hash_map::DefaultHasher;
      use std::hash::{Hash, Hasher};
      assert_eq!(align_of::<$t>(), $c);
      assert_eq!(size_of::<$t>(), $c);
      assert_eq!(align_of::<$t>(), size_of::<$t>());
      assert_eq!(size_of_val(&*<$t>::default()), size_of::<$t>());
      assert_eq!(align_of::<$t>(), size_of::<$bit>());
      assert_eq!(size_of::<$t>(), size_of::<$bit>());
      assert_eq!(size_of_val(&*<$t>::default()), size_of_val(&*<$bit>::default()));
      assert_eq!(<$t>::default().as_bytes().len(), $c);
      assert_eq!(<$t>::default().as_bytes_mut().len(), $c);
      let vec_length = 17;
      let mut v = std::vec::Vec::with_capacity(vec_length);
      (0..vec_length).for_each(|_|v.push(<$t>::default()));
      let mut v = vec![<$t>::default(); vec_length];
      assert_eq!(v.len(), vec_length);
      let mut v_ref: &mut [$t] = &mut v;
      assert_eq!(v_ref.as_bytes().len(), $c * vec_length);
      assert_eq!(v_ref.as_bytes_mut().len(), $c * vec_length);
      let v_big = vec![42_u8; 18];
      let v = (0_usize..$c).map(|i|i as u8).collect::<std::vec::Vec<_>>();
      // test TryFrom impl
      let a = $t::try_from(&v[..]).expect("TryFrom should work for a slice of the same length");
      let err = $t::try_from(&v_big[..]).unwrap_err();
      assert_eq!(err.expected_len(), $c);
      assert_eq!(err.actual_len(), 18);
      // test Clone impl
      let mut clone = a.clone();
      assert_eq!(&*a, &*clone);
      assert_eq!(a, clone);
      let mut hasher = DefaultHasher::new();
      a.hash(&mut hasher);
      let a_hash = hasher.finish();
      hasher = DefaultHasher::new();
      clone.hash(&mut hasher);
      assert_eq!(a_hash, hasher.finish());

      clone.as_bytes_mut()[3.min($c - 1)] = 42;
      assert!(a < clone);
      hasher = DefaultHasher::new();
      clone.hash(&mut hasher);
      assert_ne!(a_hash, hasher.finish());
      // Ensure proper names
      assert!(stringify!($t).ends_with(stringify!($c)));
    }
  };
}

macro_rules! impl_primitive {
  ($t:ident) => {
    impl sealed_traits::SealedAlignment for $t {}
    impl Alignment for $t {}
    impl sealed_traits::SealedAsBytes for $t {}
    #[allow(unsafe_code)]
    #[allow(clippy::use_self)]
    unsafe impl AsBytes for $t {
      fn as_bytes(&self) -> &[u8] {
        // allow use_self because of the warning for u8
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
      }
    }
    impl sealed_traits::SealedAsBytesMut for $t {}
    #[allow(unsafe_code)]
    #[allow(clippy::use_self)]
    unsafe impl AsBytesMut for $t {
      fn as_bytes_mut(&mut self) -> &mut [u8] {
        // allow use_self because of the warning for u8
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
       }
     }
   };
 }
 
impl_primitive! {u8}
impl_primitive! {i8}
impl_primitive! {u16}
impl_primitive! {i16}
impl_primitive! {u32}
impl_primitive! {i32}
impl_primitive! {u64}
impl_primitive! {i64}
impl_primitive! {u128}
impl_primitive! {i128}
impl_primitive! {usize}
impl_primitive! {isize}
impl_primitive! {f32}
impl_primitive! {f64}

impl_alignment_traits! {
  /// Struct representing an alignment of 1
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 1]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A1]`
  /// can be used as a buffer of bytes that will always be aligned to at least 1 byte.
  ///
  /// This struct isn't very useful and is here for completeness.
  #[derive(Copy)]
  pub struct A1 or Bit8(pub [u8; 1]);
  test = test_a1
}

 impl_alignment_traits! {
  /// Struct representing an alignment of 2
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 2]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A2]`
  /// can be used as a buffer of bytes that will always be aligned to at least 2 bytes.
   #[derive(Copy)]
   pub struct A2 or Bit16(pub [u8; 2]);
   test = test_a2
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 4
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 4]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A4]`
  /// can be used as a buffer of bytes that will always be aligned to at least 4 bytes.
   #[derive(Copy)]
   pub struct A4 or Bit32(pub [u8; 4]);
   test = test_a4
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 8
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 8]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A8]`
  /// can be used as a buffer of bytes that will always be aligned to at least 8 bytes.
   #[derive(Copy)]
   pub struct A8 or Bit64(pub [u8; 8]);
   test = test_a8
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 16
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 16]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A16]`
  /// can be used as a buffer of bytes that will always be aligned to at least 16 bytes.
   #[derive(Copy)]
   pub struct A16 or Bit128(pub [u8; 16]);
   test = test_a16
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 32
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 32]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A32]`
  /// can be used as a buffer of bytes that will always be aligned to at least 32 bytes.
   #[derive(Copy)]
   pub struct A32 or Bit256(pub [u8; 32]);
   test = test_a32
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 64
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// o`f [u8; 64]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A64]`
  /// can be used as a buffer of bytes that will always be aligned to at least 64 bytes.
   pub struct A64 or Bit512(pub [u8; 64]);
   test = test_a64
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 128
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 128]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A128]`
  /// can be used as a buffer of bytes that will always be aligned to at least 128 bytes.
   pub struct A128 or Bit1024(pub [u8; 128]);
   test = test_a128
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 256
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 256]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A256]`
  /// can be used as a buffer of bytes that will always be aligned to at least 256 bytes.
   pub struct A256 or Bit2048(pub [u8; 256]);
   test = test_a256
 }
 
 impl_alignment_traits! {
  /// Struct representing an alignment of 512
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of` [u8; 512]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A512]`
  /// can be used as a buffer of bytes that will always be aligned to at least 512 bytes.
   pub struct A512 or Bit4096(pub [u8; 512]);
   test = test_a512
 }
 
 #[cfg(feature = "align-1k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 1024
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 1024]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A1024]`
  /// can be used as a buffer of bytes that will always be aligned to at least 1024 bytes.
   pub struct A1024 or Bit8192 or A1k(pub [u8; 1024]);
   test = test_a1024
 }
 
 #[cfg(feature = "align-2k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 2048
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 2048]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A2048]`
  /// can be used as a buffer of bytes that will always be aligned to at least 2048 bytes.
   pub struct A2048 or Bit16384 or A2k(pub [u8; 2048]);
   test = test_a2048
 }
 
 #[cfg(feature = "align-4k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 4096
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 4096]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A4096]`
  /// can be used as a buffer of bytes that will always be aligned to at least 4096 bytes.
   pub struct A4096 or Bit32768 or A4k(pub [u8; 4096]);
   test = test_a4096
 }
 
 #[cfg(feature = "align-8k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 8192
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 8192]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A8192]`
  /// can be used as a buffer of bytes that will always be aligned to at least 8192 bytes.
   pub struct A8192 or Bit65536 or A8k(pub [u8; 8192]);
   test = test_a8192
 }
 
 #[cfg(feature = "align-16k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 16384
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of [`u8; 16384]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A16384]`
  /// can be used as a buffer of bytes that will always be aligned to at least 16384 bytes.
   pub struct A16384 or Bit131072 or A16k(pub [u8; 16384]);
   test = test_a16384
 }
 
 #[cfg(feature = "align-32k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 32768
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of `[u8; 32768]`. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A32768]`
  /// can be used as a buffer of bytes that will always be aligned to at least 32768 bytes.
   pub struct A32768 or Bit262144 or A32k(pub [u8; 32768]);
   test = test_a32768
 }
 
 #[cfg(feature = "align-64k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 65536
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of [u8; 65536]. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A65536]`
  /// can be used as a buffer of bytes that will always be aligned to at least 65536 bytes.
   pub struct A65536 or Bit524288 or A64k(pub [u8; 65536]);
   test = test_a65536
 }
 
 #[cfg(feature = "align-128k")]
 impl_alignment_traits! {
  /// Struct representing an alignment of 131072
  ///
  /// This implements the `Alignment` trait and can also be used as a byte buffer
  /// of [u8; 131072]. Since `AsBytes` is implemented for a `&[T: AsBytes]` a `&[A131072]`
  /// can be used as a buffer of bytes that will always be aligned to at least 131072 bytes.
  pub struct A131072 or Bit1048576 or A128K(pub [u8; 131072]);
   test = test_a131072
 }
 
 /// ```
 /// # use maligned::*;
 /// # use std::mem::*;
 /// assert_eq!(size_of::<Aligned<A256, [u8; 256]>>(), size_of::<[u8; 256]>());
 /// assert_eq!(size_of::<Aligned<A512, [u8; 81920]>>(), size_of::<[u8; 81920]>());
 /// ```