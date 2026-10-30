// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0075 (FIXED)
// Crate: branca
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: return/branca
// Commit: 019725d
// File: src/lib.rs


/*!
 Branca - Authenticated and Encrypted API tokens using modern cryptography.
 
This crate is a pure-Rust implementation of [Branca](https://branca.io)
 which allows generating authenticated and encrypted tamper-proof tokens.
 The [Branca specification](https://github.com/tuupola/branca-spec) is based on the
 [Fernet specification](https://github.com/fernet/spec/blob/master/Spec.md) and is also similar in
 
 ```rust
 extern crate branca;
extern crate getrandom;
 
 use branca::Branca;
 
 fn main() {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).unwrap();

     let mut token = Branca::new(&key).unwrap();
     let ciphertext = token.encode(b"Hello World!").unwrap();
 
 
 ```rust
 extern crate branca;
extern crate getrandom;
 
 use branca::Branca;
 
 fn main() {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).unwrap();

     let mut token = Branca::new(&key).unwrap();
 
     // You are able to use the builder to set the timestamp, ttL and the key.
 
     let ciphertext = token
                       .set_timestamp(1234567890)
                      .set_key(key.to_vec())
                       .set_ttl(300);
                       //.encode(b"Hello World!").unwrap();
 
 
 It is also possible to directly encode and decode functions without using the builder function.
 
Please note that Branca uses [Orion](https://github.com/orion-rs/orion) to generate secure random nonces
 when using the encode() and builder methods. By default, Branca does not allow setting the nonce directly
 since that there is a risk that it can be reused by the user which is a foot-gun.
 
 ```rust
 extern crate branca;
extern crate getrandom;
 
 use branca::{encode, decode};
 
let mut key = [0u8; 32];
getrandom::getrandom(&mut key).unwrap();

 let token = encode(b"Hello World!", &key, 123206400).unwrap();
 // token = "875G...p0a"
 
     /// `key` - The key to be used for encrypting and decrypting the input.
     ///
     ///```rust
    /// extern crate getrandom;
     /// extern crate branca;
     /// use branca::Branca;
     ///
     /// fn main() {
    ///        let mut key = [0u8; 32];
    ///        getrandom::getrandom(&mut key).unwrap();
    ///        let token = Branca::new(&key);
     /// }
     ///```
     pub fn new(key: &[u8]) -> Result<Branca, BrancaError> {
     ///
     /// # Example
     /// ```rust
    /// extern crate getrandom;
     /// extern crate branca;
     /// use branca::Branca;
     ///
     /// fn main() {
    ///     let mut key = [0u8; 32];
    ///     getrandom::getrandom(&mut key).unwrap();
    ///     let mut token = Branca::new(&key).unwrap();
     ///
     ///     let ciphertext = token.encode(b"Hello World!").unwrap();
     ///     // Branca token is now in 'ciphertext' as a String.
     ///
     ///# Example
     ///```rust
    /// extern crate getrandom;
     /// extern crate branca;
     /// use branca::Branca;
     ///
     /// fn main() {
    ///     let mut key = [0u8; 32];
    ///     getrandom::getrandom(&mut key).unwrap();
    ///
    ///     let mut token = Branca::new(&key).unwrap();
     ///     let crypted = token.encode(b"Hello World!").unwrap();
     ///     // Branca token is now in 'crypted' as a String.
     ///