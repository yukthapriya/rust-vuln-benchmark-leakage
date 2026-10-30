// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0079
// CVE: N/A
// CWE: N/A
// Crate: pqc_kyber
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pqc_kyber
// Extraction Method: commit_diff
// GitHub Repo: bwesterb/argyle-kyber
// Commit: b5c6ad1
// File: src/reference/poly.rs


let mut k = 0usize;
   let mut u: i16;
 
   match KYBER_POLYCOMPRESSEDBYTES {
     128 => {
       for i in 0..KYBER_N/8 {
         for j in 0..8 {
           // map to positive standard representatives
           u = a.coeffs[8*i+j];
           u += (u >> 15) & KYBER_Q as i16;
          t[j] = (((((u as u16) << 4) + KYBER_Q as u16 /2) / KYBER_Q as u16) & 15) as u8;
         }
         r[k]   = t[0] | (t[1] << 4);
         r[k+1] = t[2] | (t[3] << 4);
           // map to positive standard representatives
           u = a.coeffs[8*i+j];
           u += (u >> 15) & KYBER_Q as i16;
          t[j] = (((((u as u32) << 5) + KYBER_Q as u32/2) / KYBER_Q as u32) & 31) as u8;
         }
         r[k]   =  t[0]       | (t[1] << 5);
         r[k+1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
 //              - const poly *a:      input polynomial
 pub fn poly_tomsg(msg: &mut[u8], a: Poly)
 {
  let mut t;
 
   for i in 0..KYBER_N/8 {
     msg[i] = 0;
     for j in 0..8 {
      t  = a.coeffs[8*i+j];
      t += (t >> 15) & KYBER_Q as i16;
      t  = (((t << 1) + KYBER_Q as i16 /2) / KYBER_Q as i16) & 1;
       msg[i] |= (t << j) as u8;
     }
   }