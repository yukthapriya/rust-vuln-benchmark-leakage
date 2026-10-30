pub fn listen<A: AsyncAccept>(&self, listener: A) -> TlsListener<A, T>
    where
        T: AsyncTls<A::Connection>,
    {
        TlsListener {
            listener,
            tls: self.tls.clone(),
            waiting: FuturesUnordered::new(),
            max_handshakes: self.max_handshakes,
            timeout: self.handshake_timeout,
        }
    }