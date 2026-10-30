// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0112 (FIXED)
// Crate: buttplug
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: buttplugio/buttplug-rs
// Commit: 3218102
// File: crates/intiface_engine/src/engine.rs


.clone();
 
     if let Some(rest_port) = options.rest_api_port() {
      select! {
        _ = self.stop_token.cancelled() => {
          info!("Owner requested process exit, exiting.");
        }
        res = IntifaceRestServer::run(rest_port, server) => {
          info!("Rest API listener stopped, exiting.");
          if let Err(e) = res {
            error!("Error running Intiface Central RestAPI Server: {:?}", e);
          }
        }
      };
      if let Some(frontend) = &frontend {
        frontend.send(EngineMessage::EngineStopped {}).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        frontend.disconnect();
      }
       return Ok(());
     }