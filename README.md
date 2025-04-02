# dogs-rust
UdpSocket networking library with handshake(or bark) that integrates 5 bools.

Dogs don't shake hands like we do, they bark.

## This is in early development.

### Reciever example:
```rust
use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));

    let server = dogs::Dog::new(addr).unwrap();
    match server.bark_respond(dogs::BarkCode::empty()) {
        Ok(data) => println!("Responded to bark, got following data: {:#?}", data),
        Err(e) => println!("Error: {}", e)
    }
}
```

### Sender example:
```rust
use dogs;
use std::net::SocketAddr;

fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::CLIENT_PORT));
    let client = dogs::Dog::new(addr).unwrap();

    println!("Introducing...");

    let server_addr = SocketAddr::from(([127, 0, 0, 1], dogs::Dog::SERVER_PORT));
    match client.introduce(server_addr, dogs::BarkCode::empty()) {
        Ok(data) => println!("Introduced!\nGot following data: {:#?}", data),
        Err(e) => println!("Error: {}", e)
    }
}
```

For more advanced examples [look here!](https://github.com/Webbopwork/dogs-rust/tree/main/examples)

Add to your project by adding this to your Cargo.toml:
```rust
[dependencies]
dogs = { git = "https://github.com/Webbopwork/dogs-rust.git" }
```
