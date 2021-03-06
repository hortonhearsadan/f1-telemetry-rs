use packet::{parse_packet, Packet, UnpackError};
use std::io::ErrorKind;
use std::net::{ToSocketAddrs, UdpSocket};

pub mod packet;

pub struct Stream {
    socket: UdpSocket,
}

impl Stream {
    pub fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Stream> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Stream { socket })
    }

    pub fn next(&self) -> Result<Option<Packet>, UnpackError> {
        let mut buf = [0; 2048]; // All packets fit in 2048 bytes

        match self.socket.recv(&mut buf) {
            Ok(len) => match parse_packet(len, &buf) {
                Ok(p) => Ok(Some(p)),
                Err(e) => Err(e),
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    Ok(None)
                } else {
                    Err(UnpackError(format!("Error reading from socket: {:?}", e)))
                }
            }
        }
    }

    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }
}
