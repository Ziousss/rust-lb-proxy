use std::net::{TcpListener, TcpStream, Shutdown};
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("10.0.0.20:8080")?;
    
    for stream in listener.incoming(){
        let mut stream = stream?;
        let mut stream_clone = stream.try_clone()?;

        println!("Connection from {:?}", stream.peer_addr());
        let mut connection = TcpStream::connect("10.0.0.30:8080")?;
        println!("Now connected to {:?}",connection.peer_addr());
        let mut connection_clone = connection.try_clone()?;


        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut stream, &mut connection_clone){
                eprintln!("Upstream error: {}", e);
            }
            
            connection_clone.shutdown(Shutdown::Write).ok();
        });

        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut connection, &mut stream_clone){
                eprintln!("Downstream error: {}", e);
            }

            stream_clone.shutdown(Shutdown::Write).ok();
        });
    }

    Ok(())
}