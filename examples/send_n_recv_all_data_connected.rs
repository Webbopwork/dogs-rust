use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let client = dogs::ConnectedDog::new(addr).unwrap();

    // Can also be made from a Dog instance
    //let client = dogs::ConnectedDog::from(dogs::Dog::new(addr).unwrap());

    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    client.connect(server_addr).unwrap();

    println!("Connected to address: {:#?}", server_addr);

    println!("Sending data...");

    match process(&client) {
        Ok(_) => println!("Successfully sent and got BarkCode!"),
        Err(e) => println!("Got error: {}", e)
    }
}

fn process(dog: &dogs::ConnectedDog) -> std::io::Result<()> {
    let code = dogs::BarkCode::new(false, false, true, true, false);
    
    dog.identify_with_data(code, b"nah")?;
    let (_size, server_code) = dog.bark_peek_listen()?;
    let (msg_data, _size) = dog.get_all_data()?;

    println!("\nFrom connected address I got the following BarkCode:\n{:#?}", server_code);

    println!("\nGot UTF-8 message: {:#?}\n", std::str::from_utf8(&msg_data).expect("Couldn't make data into utf-8."));

    Ok(())
}