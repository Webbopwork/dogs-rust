use dogs;
use dogs::check::CheckableDog;
use std::net::SocketAddr;
use std::io;
use std::str::FromStr;

fn main() {
    let client_addr = SocketAddr::from(([0, 0, 0, 0], dogs::Dog::CLIENT_PORT));

    let client = dogs::ConnectedDog::new(client_addr).unwrap();

    // Check for the target address in arguments.
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Address argument recieved, continuing...");
        match process(&client, &args[1], &args[2]) {
            Ok(_) => println!("\nProcess over!"),
            Err(e) => println!("Got error: {}\nEnding process...", e)
        }
    } else {
        println!("No argument for address or path found, ending process...");
    }
}

fn process(dog: &dogs::ConnectedDog, address_string: &String, path: &String) -> io::Result<()> {
    // Create the server address.
    let mut server_addr = match SocketAddr::from_str(address_string) {
        Ok(addr) => addr,
        Err(e) => {
            println!("Got error: {}\nEnding process...", e);
            return Ok(());
        }
    };

    if server_addr.port() == 0 {
        server_addr.set_port(dogs::Dog::SERVER_PORT);
    }

    println!("Connecting to server...");
    dog.connect(server_addr)?;
    println!("Connected to server!\nSending request..");

    send_request(dog, path)?;

    let data = listen_for_response(dog)?;

    match str::from_utf8(&data) {
        Ok(text) => println!("Recieved utf-8:\n\n{}", text),
        Err(e) => println!("Failed to convert data to utf-8, got error: {}", e)
    }

    Ok(())
}

fn send_request(dog: &dogs::ConnectedDog, path: &String) -> io::Result<usize> {
    // Currently the handshake/bark data means nothing, that's why it's set to be empty. In the future it will have meaning tho.
    // Send a "path" or identifying data as additional data.
    dog.identify_with_data(dogs::BarkCode::empty(), &path.clone().into_bytes())
}

fn listen_for_response(dog: &dogs::ConnectedDog) -> io::Result<Vec<u8>> {
    // Recieves the BarkCode of the sender, with a peek as to not consume the packet. Code will be used in the future.
    let (_size, _code) = dog.bark_peek_listen()?;

    // Gets all data sent in the packet(NOT including BarkCode) as well as 2 hashes by the sender, consuming it, we can be sure that the address is the same as before because we didn't consume it before.
    let (recieved_data, hash1, hash2, data_hash) = dog.get_checker_duo()?;

    println!("Hash checks begin...\n");
    let is_hash1_same = hash1 == data_hash;
    let is_hash2_same = hash2 == data_hash;
    let is_either_hashes_same = is_hash1_same || is_hash2_same;
    let is_both_hashes_same = is_hash1_same && is_hash2_same;

    println!("Hash 1: {}\nHash 2: {}\nHash 1 or 2: {}\nHash 1 and 2: {}\n", is_hash1_same, is_hash2_same, is_either_hashes_same, is_both_hashes_same);

    Ok(recieved_data)
}