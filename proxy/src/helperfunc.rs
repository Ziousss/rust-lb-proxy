use std::{io::Error, net::TcpStream};

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
        let is_down = TcpStream::connect(address).is_err();
        result.push(is_down);
    }

    result
}
