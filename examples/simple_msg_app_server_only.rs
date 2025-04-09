use dogs;
use std::{
    thread,
    net::SocketAddr,
    sync::mpsc,
    io
};

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));
    let dog = dogs::Dog::new(addr).unwrap();

    let (send_channel, recv_channel) = mpsc::channel();

    let handler = thread::spawn(move || {
        let thread_dog: dogs::Dog = recv_channel.recv().unwrap();
        loop {
            match listener(&thread_dog) {
                Ok((client_addr, msg)) => println!("{:#?}: {}", client_addr, msg),
                Err(e) => println!("Error: {:#?}", e)
            }
        }
    });

    send_channel.send(dog).unwrap();

    println!("Listening...");

    handler.join().unwrap();
}

fn listener(dog: &dogs::Dog) -> io::Result<(SocketAddr, String)> {
    let (_size, client_addr, bark_code) = dog.bark_peek_listen()?;
    if !bark_code.opt1 {
        return Err(io::Error::new(io::ErrorKind::Other, "Could not identify as message"));
    }
    // Max message length is 255 bytes as it uses one byte to tell the length right now.
    let (bytes_len_array, _size, _addr) = dog.peek_data(1)?;
    let (msg_uncropped, _size, _addr) = dog.get_data(1 + bytes_len_array[0] as usize)?;
    let msg_data = &msg_uncropped[1..];
    Ok((client_addr, std::str::from_utf8(msg_data).unwrap_or("[ERROR CONVERTING data to utf-8]").to_owned()))
}