use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let client = dogs::Dog::new(addr).unwrap();

    println!("Introducing...");

    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));
    match client.introduce_empty(server_addr) {
        Ok(data) => println!("Introduced!\nGot following data: {:#?}", data),
        Err(e) => println!("Error: {}", e)
    }
}