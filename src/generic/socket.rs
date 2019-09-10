use std::net::{UdpSocket, SocketAddr};
use crate::utils::internet_address::InternetAddress;
use std::io::Error;

pub struct Socket {
	socket: Option<UdpSocket>,
	bind_address: InternetAddress,
}

impl Socket {
	pub fn new(bind_address: InternetAddress) -> Result<Socket, Error> {
		let udp_socket: UdpSocket = UdpSocket::bind(format!("{}:{}", bind_address.get_ip(), bind_address.get_port())).unwrap();
		if udp_socket.take_error().unwrap().is_some() {
			return Err(udp_socket.take_error().unwrap().unwrap());
		}
		let socket = Socket {
			socket: Some(udp_socket),
			bind_address,
		};
		socket.get_socket().set_nonblocking(true).expect("Failed to enter non-blocking mode");
		return Ok(socket);
	}
	pub fn get_bind_address(&self) -> &InternetAddress {
		return &self.bind_address;
	}
	pub fn get_socket(&self) -> &UdpSocket {
		return self.socket.as_ref().unwrap();
	}
	pub fn close(&mut self) {
		self.socket = None;
	}
	pub fn get_last_error(&self) -> Option<Error> {
		return self.get_socket().take_error().unwrap();
	}
	pub fn read_packet(&self/* , source : &str, port : &u16 */) -> Vec<u8> {
		let mut buffer: Vec<u8> = Vec::new();
		match self.get_socket().recv_from(&mut buffer) {
			Ok(ok) => {
				return buffer;
			},
			Err(e) => {
				println!("{:?}", e);
				return Vec::new();
			}
		}
	}
	pub fn write_packet(&self, buffer: &str, dest: &str, port: &u8) -> Result<usize, Error> {
		return self.get_socket().send_to(buffer.as_bytes(), format!("{}:{}", dest, port));
	}
}
