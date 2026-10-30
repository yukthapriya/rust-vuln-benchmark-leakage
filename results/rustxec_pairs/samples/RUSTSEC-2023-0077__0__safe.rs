pub fn handle_msg(&mut self, rx_buf: &[u8], tx_buf: &mut [u8]) -> Result<HandleMsgResult> {
        let seal_broken = "Message seal broken!";
        // length of the response. We assume no response, so None for now
        let mut len = 0;
        let mut exchanged = false;

        ensure!(!rx_buf.is_empty(), "received empty message, ignoring it");

        let peer = match rx_buf[0].try_into() {
            Ok(MsgType::InitHello) => {
                let msg_in = rx_buf.envelope::<InitHello<&[u8]>>()?;
                ensure!(msg_in.check_seal(self)?, seal_broken);

                let mut msg_out = tx_buf.envelope_truncating::<RespHello<&mut [u8]>>()?;
                let peer = self.handle_init_hello(
                    msg_in.payload().init_hello()?,
                    msg_out.payload_mut().resp_hello()?,
                )?;
                len = self.seal_and_commit_msg(peer, MsgType::RespHello, msg_out)?;
                peer
            }
            Ok(MsgType::RespHello) => {
                let msg_in = rx_buf.envelope::<RespHello<&[u8]>>()?;
                ensure!(msg_in.check_seal(self)?, seal_broken);

                let mut msg_out = tx_buf.envelope_truncating::<InitConf<&mut [u8]>>()?;
                let peer = self.handle_resp_hello(
                    msg_in.payload().resp_hello()?,
                    msg_out.payload_mut().init_conf()?,
                )?;
                len = self.seal_and_commit_msg(peer, MsgType::InitConf, msg_out)?;
                peer.hs()
                    .store_msg_for_retransmission(self, &tx_buf[..len])?;
                exchanged = true;
                peer
            }
            Ok(MsgType::InitConf) => {
                let msg_in = rx_buf.envelope::<InitConf<&[u8]>>()?;
                ensure!(msg_in.check_seal(self)?, seal_broken);

                let mut msg_out = tx_buf.envelope_truncating::<EmptyData<&mut [u8]>>()?;
                let peer = self.handle_init_conf(
                    msg_in.payload().init_conf()?,
                    msg_out.payload_mut().empty_data()?,
                )?;
                len = self.seal_and_commit_msg(peer, MsgType::EmptyData, msg_out)?;
                exchanged = true;
                peer
            }
            Ok(MsgType::EmptyData) => {
                let msg_in = rx_buf.envelope::<EmptyData<&[u8]>>()?;
                ensure!(msg_in.check_seal(self)?, seal_broken);

                self.handle_resp_conf(msg_in.payload().empty_data()?)?
            }
            Ok(MsgType::DataMsg) => bail!("DataMsg handling not implemented!"),
            Ok(MsgType::CookieReply) => bail!("CookieReply handling not implemented!"),
            Err(_) => {
                bail!("CookieReply handling not implemented!")
            }
        };

        Ok(HandleMsgResult {
            exchanged_with: exchanged.then_some(peer),
            resp: if len == 0 { None } else { Some(len) },
        })
    }