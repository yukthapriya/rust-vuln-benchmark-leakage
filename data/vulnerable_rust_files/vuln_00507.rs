// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0036
// CVE: N/A
// CWE: N/A
// Crate: surf
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/surf
// Extraction Method: repo_search
// GitHub Repo: http-rs/surf
// Commit: de0e5e5
// File: src/client.rs


/// ```
     pub async fn recv_bytes(&self, req: impl Into<Request>) -> Result<Vec<u8>> {
         let mut res = self.send(req.into()).await?;
        Ok(res.body_bytes().await?)
     }
 
     /// Submit a `Request` and get the response body as a string.
     /// ```
     pub async fn recv_string(&self, req: impl Into<Request>) -> Result<String> {
         let mut res = self.send(req.into()).await?;
        Ok(res.body_string().await?)
     }
 
     /// Submit a `Request` and decode the response body from json into a struct.
         req: impl Into<Request>,
     ) -> Result<T> {
         let mut res = self.send(req.into()).await?;
        Ok(res.body_json::<T>().await?)
     }
 
     /// Submit a `Request` and decode the response body from form encoding into a struct.
         req: impl Into<Request>,
     ) -> Result<T> {
         let mut res = self.send(req.into()).await?;
        Ok(res.body_form::<T>().await?)
     }
 
     /// Perform an HTTP `GET` request using the `Client` connection.