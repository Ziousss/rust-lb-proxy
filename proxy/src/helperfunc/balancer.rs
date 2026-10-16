use std::eprintln;
use std::sync::Mutex;
use tokio::net::TcpStream;

use crate::helperfunc::helperfunctions::connect_backend_timeout;

pub async fn backend_choice(
    backends: &[String],
    down: &Mutex<Vec<bool>>,
    start: usize,
    connection_timeout: u64,
) -> Option<(TcpStream, usize)> {
    for attempt in 0..backends.len() {
        let index = (start + attempt) % backends.len();
        if down.lock().unwrap()[index] {
            continue;
        }

        match connect_backend_timeout(&backends[index], connection_timeout).await {
            Ok(b) => return Some((b, index)),
            Err(e) => {
                eprintln!("{} failed: {}", backends[index], e);
                down.lock().unwrap()[index] = true;
            }
        }
    }

    None
}
