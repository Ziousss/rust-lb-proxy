use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::net::TcpStream;

use crate::{
    helperfunc::helperfunctions::connect_backend_timeout,
    structs::{Backend, ProxyState},
};

pub async fn backend_choice(state: &ProxyState) -> Option<(TcpStream, Arc<Backend>)> {
    let backends = &state.backends;
    let connection_timeout = state.connect_timeout;

    let mut candidates: Vec<(usize, &Arc<Backend>)> = Vec::new();

    for i in 0..backends.len() {
        let b = &state.backends[i];
        if b.is_available() {
            let count = b.active.load(Ordering::Relaxed);
            candidates.push((count, b));
        }
    }

    candidates.sort_by_key(|(count, _)| *count);

    for (_, b) in candidates {
        match connect_backend_timeout(&b.addr, connection_timeout).await {
            Ok(stream) => return Some((stream, Arc::clone(b))),
            Err(e) => {
                eprintln!("{} failed: {}", b.addr, e);
                b.mark_failed();
            }
        }
    }

    None
}
