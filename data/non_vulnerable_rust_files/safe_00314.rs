// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0085 (FIXED)
// Crate: matrix-sdk-crypto
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: matrix-org/matrix-rust-sdk
// Commit: 093fb5d
// File: crates/matrix-sdk-crypto/src/gossiping/machine.rs


}
     }
 
    async fn should_accept_forward(
        &self,
        info: &GossipRequest,
        sender_key: Curve25519PublicKey,
    ) -> Result<bool, CryptoStoreError> {
        let device =
            self.store.get_device_from_curve_key(&info.request_recipient, sender_key).await?;

        if let Some(device) = device {
            Ok(device.user_id() == self.user_id() && device.is_verified())
        } else {
            Ok(false)
        }
    }

     /// Receive a forwarded room key event that was sent using any of our
     /// supported content types.
     async fn receive_supported_keys(
         let algorithm = event.content.algorithm();
 
         if let Some(info) = self.get_key_info(event).await? {
            if self.should_accept_forward(&info, sender_key).await? {
                self.accept_forwarded_room_key(&info, &event.sender, sender_key, algorithm, content)
                    .await
            } else {
                warn!(
                    sender = %event.sender,
                    %sender_key,
                    room_id = %content.room_id,
                    session_id = content.session_id.as_str(),
                    claimed_sender_key = %content.claimed_sender_key,
                    "Received a forwarded room key from an unknown device, or \
                     from a device that the key request recipient doesn't own",
                );

                Ok(None)
            }
         } else {
             warn!(
                 sender = %event.sender,