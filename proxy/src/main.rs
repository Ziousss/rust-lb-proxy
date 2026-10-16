use std::sync::{Arc, Mutex};
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;

mod config;
mod helperfunc;
use crate::config::read_config::read_config;
use crate::helperfunc::balancer::backend_choice;
use crate::helperfunc::helperfunctions::health_check;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/config/config.txt");
    let config = read_config(config_path);

    let backends = config.backends;
    let backends_for_check = backends.clone();

    let down_backend = Arc::new(Mutex::new(vec![false; backends.len()]));
    let down_for_check = Arc::clone(&down_backend);

    let timeout_secs = config.connect_timeout_secs;
    let interval_secs = config.check_interval_secs;

    let mut connection_count = 0;

    tokio::spawn(health_check(
        down_for_check,
        backends_for_check,
        timeout_secs,
        interval_secs,
    ));

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
        let start = connection_count;
        connection_count = (connection_count + 1) % backends.len();

        let (mut backend, used_backend) =
            match backend_choice(&backends, &down_backend, start, timeout_secs).await {
                Some(b) => b,
                None => {
                    eprintln!("All backends are down.");
                    continue;
                }
            };

        connection_count = (used_backend + 1) % backends.len();

        println!("Connecting to {:?}", backend.peer_addr());

        // Upstream and downstream using tokio: Copies bytes from the client to the backend and conversily
        tokio::spawn(async move {
            let mut client = client;
            if let Err(e) = copy_bidirectional(&mut client, &mut backend).await {
                eprintln!("proxy error: {}", e);
            }
        });
    }
}
