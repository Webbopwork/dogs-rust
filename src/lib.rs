use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::io;

mod error; 

pub struct BarkCode {
    pub opt1: bool,
    pub opt2: bool,
    pub opt3: bool,
    pub opt4: bool,
    pub opt5: bool
}

pub struct Dog {
    pub socket: UdpSocket
}

pub struct ConnectedDog {
    pub socket: UdpSocket
}

impl BarkCode {
    // ((b, B), (a, A, o, O), (r, R), (k, K))
    pub const ENCODING: ((u8, u8), (u8, u8, u8, u8), (u8, u8), (u8, u8)) = ((98, 66), (97, 65, 111, 79), (114, 82), (107, 75));

    pub fn new(opt1: bool, opt2: bool, opt3: bool, opt4: bool, opt5: bool) -> Self {
        Self {
            opt1,
            opt2,
            opt3,
            opt4,
            opt5
        }
    }

    pub fn empty() -> Self {
        Self::new(false, false, false, false, false)
    }

    fn new_clumped(opt1: bool, opt2_opt3: (bool, bool), opt4: bool, opt5: bool) -> Self {
        Self {
            opt1,
            opt2: opt2_opt3.0,
            opt3: opt2_opt3.1,
            opt4,
            opt5
        }
    }

    pub fn decode(buffer: [u8; 4]) -> io::Result<Self> {
        Ok(Self::new_clumped(
            if buffer[0] == Self::ENCODING.0.0 {true}
            else if buffer[0] == Self::ENCODING.0.1 {false}
            else {return Err(error::throw_error("Bark buffer index 0 faulty."))},

            if buffer[1] == Self::ENCODING.1.0 {(true, true)}
            else if buffer[1] == Self::ENCODING.1.1 {(false, false)}
            else if buffer[1] == Self::ENCODING.1.2 {(true, false)}
            else if buffer[1] == Self::ENCODING.1.3 {(false, true)}
            else {return Err(error::throw_error("Bark buffer index 1 faulty."))},

            if buffer[2] == Self::ENCODING.2.0 {true}
            else if buffer[2] == Self::ENCODING.2.1 {false}
            else {return Err(error::throw_error("Bark buffer index 2 faulty."))},

            if buffer[3] == Self::ENCODING.3.0 {true}
            else if buffer[3] == Self::ENCODING.3.1 {false}
            else {return Err(error::throw_error("Bark buffer index 3 faulty."))}
        ))
    }

    pub fn encode(&self) -> [u8; 4] {
        [
            if self.opt1 {Self::ENCODING.0.0} else {Self::ENCODING.0.1},
            match (self.opt2, self.opt3) {
                (true, true) => Self::ENCODING.1.0,
                (false, false) => Self::ENCODING.1.1,
                (true, false) => Self::ENCODING.1.2,
                (false, true) => Self::ENCODING.1.3
            },
            if self.opt4 {Self::ENCODING.2.0} else {Self::ENCODING.2.1},
            if self.opt5 {Self::ENCODING.3.0} else {Self::ENCODING.3.1}
        ]
    }

    pub fn strip_from_data(data: &[u8]) -> &[u8] {
        &data[4..]
    }

    pub fn strip_from_data_extra_bytes(data: &[u8], extra_bytes: usize) -> &[u8] {
        &data[4+extra_bytes..]
    }

    pub fn data_buffer(size: usize) -> Box<[u8]> {
        vec![0u8; 4+size].into_boxed_slice()
    }
}

impl std::fmt::Debug for BarkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BarkCode")
            .field(&self.opt1)
            .field(&self.opt2)
            .field(&self.opt3)
            .field(&self.opt4)
            .field(&self.opt5)
            .finish()
    }
}

impl std::ops::Index<usize> for BarkCode {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.opt1,
            1 => &self.opt2,
            2 => &self.opt3,
            3 => &self.opt4,
            4 => &self.opt5,
            _ => panic!("Index outside of valid range 0 to 4")
        }
    }
}

//const UDP_DATA_MAX:u16 = 65_507u16;
const UDP_DATA_MAX_SIZE:usize = 65_507usize;

impl Dog {
    pub const SERVER_PORT: u16 = 5052;
    pub const CLIENT_PORT: u16 = 5053;

    pub const DATA_MAX:u16 = 65_503u16;

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?
        })
    }

    pub fn identify<A: ToSocketAddrs>(&self, addr: A, code: BarkCode) -> io::Result<usize> {
        self.socket.send_to(&code.encode(), addr)
    }

    pub fn identify_with_data<A: ToSocketAddrs>(&self, addr: A, code: BarkCode, data: &[u8]) -> io::Result<usize> {
        self.socket.send_to(&[&code.encode()[..], data].concat(), addr)
    }

    pub fn bark_listen(&self) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let (byte_count, src_addr) = self.socket.recv_from(&mut bark_buf)?;
        Ok((byte_count, src_addr, BarkCode::decode(bark_buf)?))
    }

    pub fn bark_peek_listen(&self) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let (byte_count, src_addr) = self.socket.peek_from(&mut bark_buf)?;
        Ok((byte_count, src_addr, BarkCode::decode(bark_buf)?))
    }

    pub fn bark_respond(&self, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let (size, addr, options) = self.bark_listen()?;
        self.identify(addr, code)?;
        Ok((size, addr, options))
    }

    pub fn bark_peek_respond(&self, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let (size, addr, options) = self.bark_peek_listen()?;
        self.identify(addr, code)?;
        Ok((size, addr, options))
    }

    pub fn introduce<A: ToSocketAddrs>(&self, addr: A, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        self.identify(addr, code)?;
        self.bark_listen()
    }

    pub fn introduce_peek<A: ToSocketAddrs>(&self, addr: A, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        self.identify(addr, code)?;
        self.bark_peek_listen()
    }

    pub fn get_data(&self, size: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = BarkCode::data_buffer(size);
        let (buf_size, addr) = self.socket.recv_from(&mut data)?;
        Ok((BarkCode::strip_from_data(&data).to_owned(), buf_size, addr))
    }

    pub fn peek_data(&self, size: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = BarkCode::data_buffer(size);
        let (buf_size, addr) = self.socket.peek_from(&mut data)?;
        Ok((BarkCode::strip_from_data(&data).to_owned(), buf_size, addr))
    }

    pub fn get_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = BarkCode::data_buffer(size);
        let (buf_size, addr) = self.socket.recv_from(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data, skip).to_owned(), buf_size, addr))
    }

    pub fn peek_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = BarkCode::data_buffer(size);
        let (buf_size, addr) = self.socket.peek_from(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data, skip).to_owned(), buf_size, addr))
    }

    pub fn get_all_data(&self) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let (buf_size, addr) = self.socket.recv_from(&mut data)?;
        Ok((BarkCode::strip_from_data(&data[..buf_size]).to_owned(), buf_size, addr))
    }

    pub fn peek_all_data(&self) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let (buf_size, addr) = self.socket.peek_from(&mut data)?;
        Ok((BarkCode::strip_from_data(&data[..buf_size]).to_owned(), buf_size, addr))
    }

    pub fn get_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let (buf_size, addr) = self.socket.recv_from(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data[..buf_size], skip).to_owned(), buf_size, addr))
    }

    pub fn peek_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize, SocketAddr)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let (buf_size, addr) = self.socket.peek_from(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data[..buf_size], skip).to_owned(), buf_size, addr))
    } 
}

impl ConnectedDog {
    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?
        })
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.socket.connect(addr)
    }

    pub fn identify(&self, code: BarkCode) -> io::Result<usize> {
        self.socket.send(&code.encode())
    }

    pub fn identify_with_data(&self, code: BarkCode, data: &[u8]) -> io::Result<usize> {
        self.socket.send(&[&code.encode()[..], data].concat())
    }

    pub fn bark_listen(&self) -> io::Result<(usize, BarkCode)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let byte_count = self.socket.recv(&mut bark_buf)?;
        Ok((byte_count, BarkCode::decode(bark_buf)?))
    }

    pub fn bark_peek_listen(&self) -> io::Result<(usize, BarkCode)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let byte_count = self.socket.peek(&mut bark_buf)?;
        Ok((byte_count, BarkCode::decode(bark_buf)?))
    }

    pub fn bark_respond(&self, code: BarkCode) -> io::Result<(usize, BarkCode)> {
        let (size, options) = self.bark_listen()?;
        self.identify(code)?;
        Ok((size, options))
    }

    pub fn bark_peek_respond(&self, code: BarkCode) -> io::Result<(usize, BarkCode)> {
        let (size, options) = self.bark_peek_listen()?;
        self.identify(code)?;
        Ok((size, options))
    }

    pub fn introduce(&self, code: BarkCode) -> io::Result<(usize, BarkCode)> {
        self.identify(code)?;
        self.bark_listen()
    }

    pub fn introduce_peek(&self, code: BarkCode) -> io::Result<(usize, BarkCode)> {
        self.identify(code)?;
        self.bark_peek_listen()
    }

    pub fn get_data(&self, size: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = BarkCode::data_buffer(size);
        let buf_size = self.socket.recv(&mut data)?;
        Ok((BarkCode::strip_from_data(&data).to_owned(), buf_size))
    }

    pub fn peek_data(&self, size: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = BarkCode::data_buffer(size);
        let buf_size = self.socket.peek(&mut data)?;
        Ok((BarkCode::strip_from_data(&data).to_owned(), buf_size))
    }

    pub fn get_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = BarkCode::data_buffer(size);
        let buf_size = self.socket.recv(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data, skip).to_owned(), buf_size))
    }

    pub fn peek_data_skip(&self, size: usize, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = BarkCode::data_buffer(size);
        let buf_size = self.socket.peek(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data, skip).to_owned(), buf_size))
    }

    pub fn get_all_data(&self) -> io::Result<(Vec<u8>, usize)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let buf_size = self.socket.recv(&mut data)?;
        Ok((BarkCode::strip_from_data(&data[..buf_size]).to_owned(), buf_size))
    }

    pub fn peek_all_data(&self) -> io::Result<(Vec<u8>, usize)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let buf_size = self.socket.peek(&mut data)?;
        Ok((BarkCode::strip_from_data(&data[..buf_size]).to_owned(), buf_size))
    }

    pub fn get_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let buf_size = self.socket.recv(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data[..buf_size], skip).to_owned(), buf_size))
    }

    pub fn peek_all_data_skip(&self, skip: usize) -> io::Result<(Vec<u8>, usize)> {
        let mut data = [0u8; UDP_DATA_MAX_SIZE];
        let buf_size = self.socket.peek(&mut data)?;
        Ok((BarkCode::strip_from_data_extra_bytes(&data[..buf_size], skip).to_owned(), buf_size))
    }
}

impl From<Dog> for ConnectedDog {
    fn from(dog: Dog) -> ConnectedDog {
        ConnectedDog {
            socket: dog.socket
        }
    }
}

impl From<ConnectedDog> for Dog {
    fn from(connected_dog: ConnectedDog) -> Dog {
        Dog {
            socket: connected_dog.socket
        }
    }
}