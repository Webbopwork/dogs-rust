use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let sender = dogs::ConnectedDog::new(addr).unwrap();

    let reciever_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    sender.connect(reciever_addr).unwrap();

    println!("Connected to reciever address: {:#?}", reciever_addr);

    println!("Sending data...");

    match process(&sender) {
        Ok(data_size) => println!("Successfully sent data of total size: {}!", data_size),
        Err(e) => println!("Got error: {}", e)
    }
}

fn process(dog: &dogs::ConnectedDog) -> std::io::Result<usize> {
    // A certain integrated bool can be designated to "more data incoming" if you wish to modify this script for larger files.
    let code = dogs::BarkCode::empty();
    
    // You can dynamically load the file too, this will incorpirate the data of the file "Sylvie.webp" during compile-time.
    dog.identify_with_data(code, &dogs::check::DataCheck::add_checker_mono(include_bytes!("Sylvie.webp")).into_boxed_slice())
}