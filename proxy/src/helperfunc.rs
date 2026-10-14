use std::{
    io::{Error, ErrorKind},
    net::SocketAddr,
    net::TcpStream,
    time::Duration,
};

pub fn ok_or_err<T>(r: Result<T, Error>) -> Option<T> {
    match r {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub fn check_backends(backends: &[&str]) -> Vec<bool> {
    let mut result = Vec::new();

    for address in backends {
        let is_down = connect_with_timeout(address).is_err();
        result.push(is_down);
    }

    result
}

pub fn connect_with_timeout(address: &str) -> Result<TcpStream, Error> {
    let socket = match address.parse::<SocketAddr>() {
        Ok(s) => s,
        Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid socket")),
    };

    TcpStream::connect_timeout(&socket, Duration::from_secs(10))
}
