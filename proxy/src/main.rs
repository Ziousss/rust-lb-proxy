use std::net::{Shutdown, TcpListener};

mod balancer;

fn main() -> std::io::Result<()> {
    let backends = ["10.0.0.30:8080", "10.0.0.31:8080", "10.0.0.32:8080"];
    let mut down_backend: Vec<usize> = Vec::new();
    let listener = TcpListener::bind("10.0.0.20:8080")?;
    let mut connection_count = 0;

    for client in listener.incoming(){
        //Choosing the backend and storing the index of down backends to facilitate the choice
        let start = connection_count;
        connection_count = (connection_count + 1)% backends.len();
        
        let mut backend = match balancer::backend_choice(&backends, &mut down_backend, start){
            Some(b) => b,
            None => {
                eprintln!("All backends are down.");
                continue;
            }
        };
        
        let mut client = client?;
        let mut client_clone = client.try_clone()?;
            
        println!("Connecting to {:?}",backend.peer_addr());
        let mut backend_clone = backend.try_clone()?;

        // Upstream: Copies bytes from the client to the backend 
        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut client, &mut backend_clone){
                eprintln!("Upstream error: {}", e);
            }
            
            backend_clone.shutdown(Shutdown::Write).ok();
        });

        // Downstream: Copies bytes from the backend to the client
        std::thread::spawn(move || {
            if let Err(e) = std::io::copy(&mut backend, &mut client_clone){
                eprintln!("Downstream error: {}", e);
            }

            client_clone.shutdown(Shutdown::Write).ok();
        });
    }

    Ok(())
}