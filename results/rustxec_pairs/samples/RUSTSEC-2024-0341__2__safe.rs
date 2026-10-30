pub fn listen<A: AsyncAccept>(&self, listener: A) -> TlsListener<A, T>
    where
        T: AsyncTls<A::Connection>,
    {
        TlsListener {
            listener,
            tls: self.tls.clone(),
            waiting: FuturesUnordered::new(),
            accept_batch_size: self.accept_batch_size,
            timeout: self.handshake_timeout,
        }
    }