// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0079 (FIXED)
// Crate: pqc_kyber
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: bwesterb/argyle-kyber
// Commit: b5c6ad1
// File: src/reference/poly.rs


let mut k = 0usize;
   let mut u: i16;
 
  // Compress_q(x, d) = ⌈(2ᵈ/q)x⌋ mod⁺ 2ᵈ
  //                  = ⌊(2ᵈ/q)x+½⌋ mod⁺ 2ᵈ
  //                  = ⌊((x << d) + q/2) / q⌋ mod⁺ 2ᵈ
  //                  = DIV((x << d) + q/2, q) & ((1<<d) - 1)
  //
  // We approximate DIV(x, q) by computing (x*a)>>e, where a/(2^e) ≈ 1/q.
  // For d in {10,11} we use 20,642,678/2^36, which computes division by x/q
  // correctly for 0 ≤ x < 41,522,616, which fits (q << 11) + q/2 comfortably.
  // For d in {4,5} we use 315/2^20, which doesn't compute division by x/q
  // correctly for all inputs, but it's close enough that the end result
  // of the compression is correct. The advantage is that we do not need
  // to use a 64-bit intermediate value.

   match KYBER_POLYCOMPRESSEDBYTES {
     128 => {
       for i in 0..KYBER_N/8 {
         for j in 0..8 {
           // map to positive standard representatives
           u = a.coeffs[8*i+j];
           u += (u >> 15) & KYBER_Q as i16;
          let mut tmp : u32 = (((u as u16) << 4) + KYBER_Q as u16 /2) as u32;
          tmp *= 315;
          tmp >>= 20;
          t[j] = ((tmp as u16) & 15) as u8;
         }
         r[k]   = t[0] | (t[1] << 4);
         r[k+1] = t[2] | (t[3] << 4);
           // map to positive standard representatives
           u = a.coeffs[8*i+j];
           u += (u >> 15) & KYBER_Q as i16;
          let mut tmp : u32 = ((u as u32) << 5) + KYBER_Q as u32/2;
          tmp *= 315;
          tmp >>= 20;
          t[j] = ((tmp as u16) & 31) as u8;
         }
         r[k]   =  t[0]       | (t[1] << 5);
         r[k+1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
 //              - const poly *a:      input polynomial
 pub fn poly_tomsg(msg: &mut[u8], a: Poly)
 {
  let mut t : u32;
 
   for i in 0..KYBER_N/8 {
     msg[i] = 0;
     for j in 0..8 {
      t  = a.coeffs[8*i+j] as u32;

      t <<= 1;
      t = t.wrapping_add(1665);
      t = t.wrapping_mul(80635);
      t >>= 28;
      t &= 1;

       msg[i] |= (t << j) as u8;
     }
   }