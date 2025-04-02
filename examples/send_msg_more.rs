use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let client = dogs::Dog::new(addr).unwrap();

    println!("Sending data...");

    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    match process(&client, server_addr) {
        Ok(_) => println!("Successfully sent and got BarkCode!"),
        Err(e) => println!("Got error: {}", e)
    }
}

fn process(dog: &dogs::Dog, server_addr: SocketAddr) -> std::io::Result<()> {
    let code = dogs::BarkCode::new(false, false, true, true, false);
    
    dog.identify_with_data(server_addr, code, b"no")?;
    let (_size, _server_addr, server_code) = dog.bark_peek_listen()?;
    let (msg_data, _size, _addr) = dog.get_data(2)?;

    println!("\nFrom {:#?} I got the following BarkCode:\n{:#?}", server_addr, server_code);

    println!("\nGot UTF-8 message: {:#?}\n", std::str::from_utf8(&msg_data).expect("Couldn't make data into utf-8."));

    Ok(())
}