use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

pub async fn connect_backend_timeout(address: &String, secs: Duration) -> Result<TcpStream, Error> {
    match timeout(secs, TcpStream::connect(address)).await {
        Ok(result) => result,
        Err(_) => Err(Error::new(ErrorKind::TimedOut, "Connect timed out")),
    }
}
