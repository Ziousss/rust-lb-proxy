use std::net::TcpStream;
use std::sync::Mutex;

pub fn backend_choice(
    backends: &[&str],
    down: &Mutex<Vec<bool>>,
    start: usize,
) -> Option<(TcpStream, usize)> {
    for attempt in 0..backends.len() {
        let index = (start + attempt) % backends.len();
        if down.lock().unwrap()[index] {
            continue;
        }

        match TcpStream::connect(backends[index]) {
            Ok(b) => return Some((b, index)),
            Err(e) => {
                eprintln!("{} failed: {}", backends[index], e);
                down.lock().unwrap()[index] = true;
            }
        }
    }

    None
}
