use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let server = dogs::Dog::new(addr).unwrap();
    match server.bark_respond() {
        Ok(_) => println!("Responded to bark"),
        Err(e) => println!("Error: {}", e)
    }
}