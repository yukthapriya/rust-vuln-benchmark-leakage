// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0085
// CVE: CVE-2022-39252
// CWE: N/A
// Crate: matrix-sdk-crypto
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/matrix-sdk-crypto
// Extraction Method: commit_diff
// GitHub Repo: matrix-org/matrix-rust-sdk
// Commit: 093fb5d
// File: crates/matrix-sdk-crypto/src/gossiping/machine.rs


}
     }
 
     /// Receive a forwarded room key event that was sent using any of our
     /// supported content types.
     async fn receive_supported_keys(
         let algorithm = event.content.algorithm();
 
         if let Some(info) = self.get_key_info(event).await? {
            self.accept_forwarded_room_key(&info, &event.sender, sender_key, algorithm, content)
                .await
         } else {
             warn!(
                 sender = %event.sender,