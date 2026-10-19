use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::net::TcpStream;

use crate::{
    helperfunc::helperfunctions::connect_backend_timeout,
    structs::{Backend, ProxyState},
};

pub async fn backend_choice(state: &ProxyState) -> Option<(TcpStream, Arc<Backend>)> {
    let connection_timeout = state.connect_timeout;

    let mut candidates: Vec<(usize, &Arc<Backend>)> = Vec::new();

    for b in &state.backends {
        if b.is_available() {
            let count = b.active.load(Ordering::Relaxed);
            candidates.push((count, b));
        }
    }

    if candidates.is_empty() {
        return None;
    }

    let start = state.next.fetch_add(1, Ordering::Relaxed) % candidates.len();
    candidates.rotate_left(start);
    candidates.sort_by_key(|(count, _)| *count);

    for (_, b) in candidates {
        if !b.try_to_get() {
            continue;
        }
        match connect_backend_timeout(&b.addr, connection_timeout).await {
            Ok(stream) => {
                b.record_success();
                return Some((stream, Arc::clone(b)));
            }
            Err(e) => {
                eprintln!("{} failed: {}", b.addr, e);
                if let Some(d) = b.record_failure() {
                    println!("Backend {} ejected for {:?}", b.addr, d);
                }
            }
        }
    }

    None
}
