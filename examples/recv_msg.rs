use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let server = dogs::Dog::new(addr).unwrap();

    loop {
        println!("Listening for connection...");
        match process(&server) {
            Ok(_) => println!("Successfully responded with BarkCode!"),
            Err(e) => println!("Got error: {}", e)
        }
    }
}

fn process(dog: &dogs::Dog) -> std::io::Result<()> {
    let (_size, client_addr, _code) = dog.bark_peek_listen()?;
    
    let mut data: [u8; 6] = [0u8; 6];
    dog.socket.recv_from(&mut data)?;

    let msg_data = dogs::BarkCode::strip_from_data(&data);

    println!("\nGot UTF-8 message: {:#?}\n", std::str::from_utf8(&msg_data).expect("Couldn't make data into utf-8."));

    let response_code = dogs::BarkCode::empty();
    
    dog.identify_with_data(client_addr, response_code, b"ok")
}