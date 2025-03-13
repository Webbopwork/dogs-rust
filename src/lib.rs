use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::io;

pub struct Dog {
    socket: UdpSocket
}

impl Dog {
    pub const SERVER_PORT: u16 = 5052;
    pub const CLIENT_PORT: u16 = 5053;

    // ((b, B), (a, A), (r, R), (k, K))
    pub const BARK_CODES: ((u8, u8), (u8, u8), (u8, u8), (u8, u8)) = ((98, 66), (97, 65), (114, 82), (107, 75));

    pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(addr)?
        })
    }

    pub fn bark_encode(opt1: bool, opt2: bool, opt3: bool, opt4: bool) -> [u8; 4] {
        [
            if opt1 {Self::BARK_CODES.0.0} else {Self::BARK_CODES.0.1},
            if opt2 {Self::BARK_CODES.1.0} else {Self::BARK_CODES.1.1},
            if opt3 {Self::BARK_CODES.2.0} else {Self::BARK_CODES.2.1},
            if opt4 {Self::BARK_CODES.3.0} else {Self::BARK_CODES.3.1}
        ]
    }

    pub fn identify<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.socket.send_to(&Self::bark_encode(false, false, false, false), addr)?;
        Ok(())
    }

    pub fn bark_listen(&self) -> io::Result<(usize, SocketAddr)> {
        let mut bark_buf: [u8; 4] = [0u8; 4];
        let (byte_count, src_addr) = self.socket.recv_from(&mut bark_buf)?;
        Ok((byte_count, src_addr))
    }

    pub fn bark_respond(&self) -> io::Result<()> {
        //let (_, addr, _) = self.bark_listen()?;
        let (_, addr) = self.bark_listen()?;
        self.identify(addr)
    }

    pub fn introduce<A: ToSocketAddrs>(&self, addr: A) -> io::Result<(usize, SocketAddr)> {
        self.identify(addr)?;
        self.bark_listen()
    }
}