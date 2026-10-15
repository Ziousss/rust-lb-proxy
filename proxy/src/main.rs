use std::net::{Shutdown, TcpListener};
use std::println;
use std::sync::{Arc, Mutex};

mod config;
mod helperfunc;
use crate::config::config::read_config;
use crate::helperfunc::balancer::backend_choice;
use crate::helperfunc::helperfunc::{health_check, ok_or_err};

fn main() -> std::io::Result<()> {
    let config = read_config("config/config.txt");

    let backends = config.backends;
    let backends_for_check = backends.clone();

    let down_backend = Arc::new(Mutex::new(vec![false; backends.len()]));
    let down_for_check = Arc::clone(&down_backend);

    let listener = TcpListener::bind(config.bind)?;

    let mut connection_count = 0;

    std::thread::spawn(move || {
        health_check(down_for_check, backends_for_check);
    });

    for client in listener.incoming() {
        // Make sure client is accepted
        let mut client = match ok_or_err(client) {
            Some(c) => c,
            None => continue,
        };

        let mut client_clone = match ok_or_err(client.try_clone()) {
            Some(c) => c,
            None => continue,
        };

        // Choosing the backend and storing the index of down backends to facilitate the choice
        let start = connection_count;
        connection_count = (connection_count + 1) % backends.len();

        let (mut backend, used_backend) = match backend_choice(&backends, &down_backend, start) {
            Some(b) => b,
            None => {
                eprintln!("All backends are down.");
                continue;
            }
        };

        connection_count = (used_backend + 1) % backends.len();

        println!("Connecting to {:?}", backend.peer_addr());
        let mut backend_clone = match ok_or_err(backend.try_clone()) {
            Some(c) => c,
            None => continue,
        };

        // Upstream: Copies bytes from the client to the backend
        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut client, &mut backend_clone) {
                eprintln!("Upstream error: {}", e);
            }

            backend_clone.shutdown(Shutdown::Write).ok();
        });

        // Downstream: Copies bytes from the backend to the client
        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut backend, &mut client_clone) {
                eprintln!("Downstream error: {}", e);
            }

            client_clone.shutdown(Shutdown::Write).ok();
        });
    }

    Ok(())
}
