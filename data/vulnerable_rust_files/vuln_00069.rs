// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0031
// CVE: CVE-2020-35884
// CWE: N/A
// Crate: tiny_http
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tiny_http
// Extraction Method: repo_search
// GitHub Repo: tiny-http/tiny-http
// Commit: 876efd6
// File: src/ssl/openssl.rs


let mut ctx = openssl::ssl::SslContext::builder(ssl::SslMethod::tls())?;
         ctx.set_cipher_list("DEFAULT")?;
        let cert = X509::from_pem(&certificates)?;
        ctx.set_certificate(&cert)?;
         let key = PKey::private_key_from_pem(&private_key)?;
         ctx.set_private_key(&key)?;
         ctx.set_verify(SslVerifyMode::NONE);