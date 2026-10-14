use std::net::TcpStream;

pub fn backend_choice(backends: &[&str], down: &mut Vec<usize>, start: usize,) -> Option<TcpStream>{
    for attempt in 0..backends.len() {
        let index = (start + attempt) % backends.len();
        if down.contains(&index){
            continue;
        }

        match TcpStream::connect(backends[index]){
            Ok(b) => return Some(b),
            Err(e) => {
                eprintln!("{} failed: {}", backends[index], e);
                down.push(index);
            }
        }
    }

    None
}