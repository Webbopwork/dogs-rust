use dogs;
use dogs::check::CheckableDog;
use std::net::SocketAddr;
use std::io::prelude::*;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let reciever = dogs::Dog::new(addr).unwrap();

    loop {
        println!("Listening for sender connection...");
        match process(&reciever) {
            Ok(_) => println!("Successfully got and wrote down data!\nThe data should be in the file named 'new-recvd-Sylvie.webp'."),
            Err(e) => println!("Got error: {}", e)
        }
    }
}

fn process(dog: &dogs::Dog) -> std::io::Result<()> {
    // Recieves the address and BarkCode of the sender, with a peek as to not consume the packet.
    let (_size, sender_addr, code) = dog.bark_peek_listen()?;

    // Gets all data sent in the packet(NOT including BarkCode) as well as 1 hash by the sender, consuming it, we can be sure that the address is the same as before because we didn't consume it before.
    let (recieved_data, hash, data_hash) = dog.get_checker_mono()?;
    let same_data = hash == data_hash;

    println!("\nFrom {:#?} I got the following BarkCode:\n{:#?}\nIs data the same: {:#?}", sender_addr, code, same_data);

    // Creates a file to store the data in, you can name it anything you want.
    let mut file = std::fs::File::create("new-recvd-Sylvie.webp")?;

    // Writing down the data recieved by the sender into the file.
    file.write_all(&recieved_data.into_boxed_slice())?;

    Ok(())
}