use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let server = dogs::Dog::new(addr).unwrap();
    match server.bark_respond_empty() {
        Ok(data) => println!("Responded to bark, got following data: {:#?}", data),
        Err(e) => println!("Error: {}", e)
    }
}