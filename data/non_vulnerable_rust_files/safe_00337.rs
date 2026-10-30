// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0027 (FIXED)
// Crate: async-nats
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: nats-io/nats.rs
// Commit: 817a7b9
// File: async-nats/src/connector.rs


})
                 .map_err(|err| ConnectError::with_source(crate::ConnectErrorKind::Tls, err))?;
 
            let domain = rustls::ServerName::try_from(tls_host)
                .map_err(|err| ConnectError::with_source(crate::ConnectErrorKind::Tls, err))?;
 
             connection = Connection {
                 stream: Box::new(tls_connector.connect(domain, connection.stream).await?),