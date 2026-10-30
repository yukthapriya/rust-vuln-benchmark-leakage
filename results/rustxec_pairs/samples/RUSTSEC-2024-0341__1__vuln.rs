async fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3000).into();
    let counter = Arc::new(AtomicU64::new(0));

    let mut listener = tls_listener::builder(tls_acceptor())
        .max_handshakes(10)
        .listen(TcpListener::bind(addr).await.expect("Failed to bind port"));

    let (tx, mut rx) = mpsc::channel::<Acceptor>(1);

    let http = http1::Builder::new();
    loop {
        tokio::select! {
            conn = listener.accept() => {
                match conn {
                    Ok((conn, remote_addr)) => {
                        let http = http.clone();
                        let tx = tx.clone();
                        let counter = counter.clone();
                        tokio::spawn(async move {
                            let svc = service_fn(move |request| handle_request(tx.clone(), counter.clone(), request));
                            if let Err(err) = http.serve_connection(TokioIo::new(conn), svc).await {
                                eprintln!("Application error (client address: {remote_addr}): {err}");
                            }
                        });
                    },
                    Err(e) => {
                        if let Some(remote_addr) = e.peer_addr() {
                            eprint!("[client {remote_addr}] ");
                        }

                        eprintln!("Bad connection: {}", e);
                    }
                }
            },
            message = rx.recv() => {
                // Certificate is loaded on another task; we don't want to block the listener loop
                let acceptor = message.expect("Channel should not be closed");
                println!("Rotating certificate...");
                listener.replace_acceptor(acceptor);
            }
        }
    }
}