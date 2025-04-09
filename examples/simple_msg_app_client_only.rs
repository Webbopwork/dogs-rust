use dogs;
use std::net::SocketAddr;

fn main() {
    // With the IP 127.0.0.1 it will connect to your current IP, meaning it's looking for a server on the same device, change it to a different one to connect to a remote server.
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let client = dogs::ConnectedDog::new(addr).unwrap();

    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));
    client.connect(server_addr).unwrap();

    println!("Connected to address: {:#?}", server_addr);

    println!("\nSend message: ");
    loop {
        match process(&client) {
            Ok(_) => println!("[Message sent]"),
            Err(e) => println!("Got error: {}", e)
        }
    }
}

fn process(dog: &dogs::ConnectedDog) -> std::io::Result<()> {
    let code = dogs::BarkCode::new(true, false, false, false, false);

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(length) => dog.identify_with_data(code, &[&[length as u8], input.as_bytes()].concat())?,
        Err(e) => {
            println!("Error: {}", e);
            0usize
        }
    };

    Ok(())
}