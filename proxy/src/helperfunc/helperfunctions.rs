use std::sync::Arc;
use std::{
    io::{Error, ErrorKind},
    sync::Mutex,
};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

pub async fn check_backends(backends: &[String], secs: u64) -> Vec<bool> {
    let mut result = Vec::new();

    for address in backends {
        let is_down = connect_backend_timeout(address, secs).await.is_err();
        result.push(is_down);
    }

    result
}

pub async fn connect_backend_timeout(address: &String, secs: u64) -> Result<TcpStream, Error> {
    match timeout(Duration::from_secs(secs), TcpStream::connect(address)).await {
        Ok(result) => result,
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connect timed out")),
    }
}

pub async fn health_check(
    down_for_check: Arc<Mutex<Vec<bool>>>,
    backends_for_check: Vec<String>,
    check_timeout: u64,
    interval_secs: u64,
) {
    loop {
        let backend_results = check_backends(&backends_for_check, check_timeout).await;
        *down_for_check.lock().unwrap() = backend_results;

        println!("Check done.");
        tokio::time::sleep(Duration::from_secs(interval_secs)).await;
    }
}
