use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::io;

mod error; 

pub struct Dog {
    socket: UdpSocket
}

impl Dog {
    pub const SERVER_PORT: u16 = 5052;
    pub const CLIENT_PORT: u16 = 5053;

    // ((b, B), (a, A, o, O), (r, R), (k, K))
    pub const BARK_CODES: ((u8, u8), (u8, u8, u8, u8), (u8, u8), (u8, u8)) = ((98, 66), (97, 65, 111, 79), (114, 82), (107, 75));

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?
        })
    }

    pub fn bark_encode(opt1: bool, opt2: bool, opt3: bool, opt4: bool, opt5: bool) -> [u8; 4] {
        [
            if opt1 {Self::BARK_CODES.0.0} else {Self::BARK_CODES.0.1},
            match (opt2, opt3) {
                (true, true) => Self::BARK_CODES.1.0,
                (false, false) => Self::BARK_CODES.1.1,
                (true, false) => Self::BARK_CODES.1.2,
                (false, true) => Self::BARK_CODES.1.3
            },
            if opt4 {Self::BARK_CODES.2.0} else {Self::BARK_CODES.2.1},
            if opt5 {Self::BARK_CODES.3.0} else {Self::BARK_CODES.3.1}
        ]
    }

    pub fn bark_decode(buffer: [u8; 4]) -> io::Result<(bool, (bool, bool), bool, bool)> {
        Ok((
            if buffer[0] == Self::BARK_CODES.0.0 {true}
            else if buffer[0] == Self::BARK_CODES.0.1 {false}
            else {return Err(error::thow_error("Bark buffer index 0 faulty."))},

            if buffer[1] == Self::BARK_CODES.1.0 {(true, true)}
            else if buffer[1] == Self::BARK_CODES.1.1 {(false, false)}
            else if buffer[1] == Self::BARK_CODES.1.2 {(true, false)}
            else if buffer[1] == Self::BARK_CODES.1.3 {(false, true)}
            else {return Err(error::thow_error("Bark buffer index 1 faulty."))},

            if buffer[2] == Self::BARK_CODES.2.0 {true}
            else if buffer[2] == Self::BARK_CODES.2.1 {false}
            else {return Err(error::thow_error("Bark buffer index 2 faulty."))},

            if buffer[3] == Self::BARK_CODES.3.0 {true}
            else if buffer[3] == Self::BARK_CODES.3.1 {false}
            else {return Err(error::thow_error("Bark buffer index 3 faulty."))}
        ))
    }

    pub fn identify<A: ToSocketAddrs>(&self, addr: A, opt1: bool, opt2: bool, opt3: bool, opt4: bool, opt5: bool) -> io::Result<()> {
        self.socket.send_to(&Self::bark_encode(opt1, opt2, opt3, opt4, opt5), addr)?;
        Ok(())
    }

    pub fn bark_listen(&self) -> io::Result<(usize, SocketAddr, (bool, (bool, bool), bool, bool))> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let (byte_count, src_addr) = self.socket.recv_from(&mut bark_buf)?;
        Ok((byte_count, src_addr, Self::bark_decode(bark_buf)?))
    }

    pub fn bark_respond(&self, opt1: bool, opt2: bool, opt3: bool, opt4: bool, opt5: bool) -> io::Result<(usize, SocketAddr, (bool, (bool, bool), bool, bool))> {
        let (size, addr, options) = self.bark_listen()?;
        self.identify(addr, opt1, opt2, opt3, opt4, opt5)?;
        Ok((size, addr, options))
    }

    pub fn bark_respond_empty(&self) -> io::Result<(usize, SocketAddr, (bool, (bool, bool), bool, bool))> {
        self.bark_respond(false, false, false, false, false)
    }

    pub fn introduce<A: ToSocketAddrs>(&self, addr: A, opt1: bool, opt2: bool, opt3: bool, opt4: bool, opt5: bool) -> io::Result<(usize, SocketAddr, (bool, (bool, bool), bool, bool))> {
        self.identify(addr, opt1, opt2, opt3, opt4, opt5)?;
        self.bark_listen()
    }

    pub fn introduce_empty<A: ToSocketAddrs>(&self, addr: A) -> io::Result<(usize, SocketAddr, (bool, (bool, bool), bool, bool))> {
        self.introduce(addr, false, false, false, false, false)
    }
}