use std::{net::{TcpListener, TcpStream}, println};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("10.0.0.20:8080")?;
    
    for stream in listener.incoming(){
        let stream = stream?;
        println!("Connection from {:?}", stream.peer_addr());
    }

    Ok(())
}
