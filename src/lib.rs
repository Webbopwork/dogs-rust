use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::io;

mod error; 

pub struct BarkCode {
    opt1: bool,
    opt2: bool,
    opt3: bool,
    opt4: bool,
    opt5: bool
}

pub struct Dog {
    socket: UdpSocket
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
}

impl std::fmt::Debug for BarkCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.opt1)
            .field(&self.opt2)
            .field(&self.opt3)
            .field(&self.opt4)
            .field(&self.opt5)
            .finish()
    }
}

impl Dog {
    pub const SERVER_PORT: u16 = 5052;
    pub const CLIENT_PORT: u16 = 5053;

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?
        })
    }

    pub fn identify<A: ToSocketAddrs>(&self, addr: A, code: BarkCode) -> io::Result<()> {
        self.socket.send_to(&code.encode(), addr)?;
        Ok(())
    }

    pub fn bark_listen(&self) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let (byte_count, src_addr) = self.socket.recv_from(&mut bark_buf)?;
        Ok((byte_count, src_addr, BarkCode::decode(bark_buf)?))
    }

    pub fn bark_respond(&self, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        let (size, addr, options) = self.bark_listen()?;
        self.identify(addr, code)?;
        Ok((size, addr, options))
    }

    pub fn introduce<A: ToSocketAddrs>(&self, addr: A, code: BarkCode) -> io::Result<(usize, SocketAddr, BarkCode)> {
        self.identify(addr, code)?;
        self.bark_listen()
    }
}