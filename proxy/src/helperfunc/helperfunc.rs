use std::sync::Arc;
use std::thread::sleep;
use std::{
    io::{Error, ErrorKind},
    net::{SocketAddr, TcpStream},
    sync::Mutex,
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

pub fn check_backends(backends: &[String]) -> Vec<bool> {
    let mut result = Vec::new();

    for address in backends {
        let is_down = connect_with_timeout(&address).is_err();
        result.push(is_down);
    }

    result
}

pub fn connect_with_timeout(address: &String) -> Result<TcpStream, Error> {
    let socket = match address.parse::<SocketAddr>() {
        Ok(s) => s,
        Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid socket")),
    };

    TcpStream::connect_timeout(&socket, Duration::from_secs(10))
}

pub fn health_check(down_for_check: Arc<Mutex<Vec<bool>>>, backends_for_check: Vec<String>) {
    loop {
        let backend_results = check_backends(&backends_for_check);
        *down_for_check.lock().unwrap() = backend_results;

        println!("Check done.");
        sleep(Duration::from_secs(5));
    }
}
