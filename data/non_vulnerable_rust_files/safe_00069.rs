// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0031 (FIXED)
// Crate: tiny_http
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: tiny-http/tiny-http
// Commit: 876efd6
// File: src/ssl/openssl.rs


let mut ctx = openssl::ssl::SslContext::builder(ssl::SslMethod::tls())?;
         ctx.set_cipher_list("DEFAULT")?;
        let certificate_chain = X509::stack_from_pem(&certificates)?;
        if certificate_chain.is_empty() {
            return Err("Couldn't extract certificate chain from config.".into());
        }
        // The leaf certificate must always be first in the PEM file
        ctx.set_certificate(&certificate_chain[0])?;
        for chain_cert in certificate_chain.into_iter().skip(1) {
            ctx.add_extra_chain_cert(chain_cert)?;
        }
         let key = PKey::private_key_from_pem(&private_key)?;
         ctx.set_private_key(&key)?;
         ctx.set_verify(SslVerifyMode::NONE);