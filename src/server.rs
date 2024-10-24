use crate::result::Result;
use core::str;
use std::net::{ToSocketAddrs, UdpSocket};

pub struct Server {
	socket: UdpSocket,
}

impl Server {
	pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
		let socket = UdpSocket::bind(addr)?;
		Ok(Self { socket })
	}

	pub fn listen(&self) -> Result<()> {
		let mut buf = [0; 2048];
		loop {
			let (recv_len, other_addr) = self.socket.recv_from(&mut buf)?;
			let msg = &buf[..recv_len];
			if let Ok(s) = str::from_utf8(msg) {
				print!("message! {s}");
				let reply = format!("Received: {s}");
				self.socket.send_to(reply.as_bytes(), other_addr)?;
			};
		}
	}
}
