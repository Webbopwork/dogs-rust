use dogs;
use std::net::SocketAddr;

fn main() {
    let server_addr = SocketAddr::from(([0, 0, 0, 0], dogs::Dog::SERVER_PORT));
    let server = dogs::Dog::new(server_addr).unwrap();

    println!("Listening for connection...");
    loop {
        match process(&server) {
            Ok(data_size) => println!("Successfully sent data of total size: {}!", data_size),
            Err(e) => println!("Got error: {}", e)
        }
    }
}

fn process(dog: &dogs::Dog) -> std::io::Result<usize> {
    // Recieves the BarkCode of the client, with peek as to not consume the packet. Code will be used in the future.
    // It also collects the address of the client.
    // in the future if we use peek we can incorpirate addresses in the request.
    let (_size, client_addr, _code) = dog.bark_peek_listen()?;
    let (path_data, _size, _client_addr) = dog.get_all_data()?;
    let mut path = String::from_utf8_lossy(&path_data);

    // A certain integrated bool can be designated to "more data incoming" if you wish to modify this script for larger files. Currently this is not used.
    let code = dogs::BarkCode::empty();
    dog.identify_with_data(client_addr, code, 
        &prepare_for_travel(
            match path.to_mut().as_str() {
                "content" => include_bytes!("content.txt"),
                "bleeh" => include_bytes!("bleeh.txt"),
                "blunt" => include_bytes!("blunt.txt"),
                _ => include_bytes!("invalid_path.txt")
            }
        )
    )
}

fn prepare_for_travel(travel_data: &[u8]) -> Box<[u8]> {
    // You can dynamically load the file too, this will incorpirate the data of the file loaded with the macro "include_bytes" during compile-time.
    dogs::check::DataCheck::add_checker_duo(travel_data).into_boxed_slice()
}