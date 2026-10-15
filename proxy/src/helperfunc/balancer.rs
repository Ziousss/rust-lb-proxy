use std::eprintln;
use std::net::TcpStream;
use std::sync::Mutex;

use crate::helperfunc::helperfunc::connect_with_timeout;

pub fn backend_choice(
    backends: &[String],
    down: &Mutex<Vec<bool>>,
    start: usize,
) -> Option<(TcpStream, usize)> {
    for attempt in 0..backends.len() {
        let index = (start + attempt) % backends.len();
        if down.lock().unwrap()[index] {
            continue;
        }

        match connect_with_timeout(&backends[index]) {
            Ok(b) => return Some((b, index)),
            Err(e) => {
                eprintln!("{} failed: {}", backends[index], e);
                down.lock().unwrap()[index] = true;
            }
        }
    }

    None
}
