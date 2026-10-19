use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;

mod config;
mod helperfunc;
mod structs;
use crate::config::read_config::read_config;
use crate::helperfunc::balancer::backend_choice;
use crate::structs::Backend;
use crate::structs::ProxyState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/config/config.txt");
    let config = read_config(config_path);

    let backends: Vec<Arc<Backend>> = config.backends.into_iter().map(Backend::new).collect();

    let state = ProxyState::new(
        backends,
        Duration::from_secs(config.connect_timeout_secs),
        //Duration::from_secs(config.check_interval_secs),
    );

    // Health check is now passive and done when trying to connect to the backends
    // Check balancer.rs

    let listener = TcpListener::bind(config.bind).await?;

    loop {
        let (client, _) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                eprintln!("Accept error: {}", e);
                continue;
            }
        };

        // Choosing the backend and storing the index of down backends to facilitate the choice
        let state = Arc::clone(&state);
        tokio::spawn(async move {
            // Choosing the backend
            let (mut stream, backend) = match backend_choice(&state).await {
                Some(pair) => pair,
                None => {
                    eprintln!("All backends are down.");
                    return;
                }
            };

            let now = backend.active.fetch_add(1, Ordering::Relaxed) + 1;
            println!(
                "Connecting to {:?}, {} total connections connections",
                backend.addr, now
            );

            let mut client = client;

            // Upstream and downstream using tokio: Copies bytes from the client to the backend and conversily
            if let Err(e) = copy_bidirectional(&mut client, &mut stream).await {
                eprintln!("proxy error: {}", e);
            }
            backend.active.fetch_sub(1, Ordering::Relaxed);
        });
    }
}
