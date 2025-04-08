use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let server = dogs::Dog::new(addr).unwrap();

    loop {
        println!("Listening for connection...");
        match process(&server) {
            Ok(_) => println!("Successfully echoed back BarkCode!"),
            Err(e) => println!("Got error: {}", e)
        }
    }
}

fn process(dog: &dogs::Dog) -> std::io::Result<usize> {
    let (_size, client_addr, code) = dog.bark_listen()?;
    dog.identify(client_addr, code)
}