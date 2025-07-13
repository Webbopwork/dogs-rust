use dogs;
use std::net::SocketAddr;

fn main() {
    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));
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
    // Recieves the BarkCode of the client, with a get as to consume the packet. Code will be used in the future.
    // It also collects the address of the client.
    // in the future if we use peek we can incorpirate addresses in the request.
    let (_size, client_addr, _code) = dog.bark_listen()?;

    // A certain integrated bool can be designated to "more data incoming" if you wish to modify this script for larger files. Currently this is not used.
    let code = dogs::BarkCode::empty();
    
    // You can dynamically load the file too, this will incorpirate the data of the file "content.txt" during compile-time.
    dog.identify_with_data(client_addr, code, &dogs::check::DataCheck::add_checker_duo(include_bytes!("content.txt")).into_boxed_slice())
}