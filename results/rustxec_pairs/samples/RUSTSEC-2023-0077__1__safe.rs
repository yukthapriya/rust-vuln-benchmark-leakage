pub fn initiate_handshake(&mut self, peer: PeerPtr, tx_buf: &mut [u8]) -> Result<usize> {
        let mut msg = tx_buf.envelope_truncating::<InitHello<()>>()?; // Envelope::<InitHello>::default(); // TODO
        self.handle_initiation(peer, msg.payload_mut().init_hello()?)?;
        let len = self.seal_and_commit_msg(peer, MsgType::InitHello, msg)?;
        peer.hs()
            .store_msg_for_retransmission(self, &tx_buf[..len])?;
        Ok(len)
    }