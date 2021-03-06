extern crate atoi;
use crate::utils::internet_address::InternetAddress;
use itoa::fmt;
use binaryutils::binary::Endian::Big;
use atoi::atoi;
use binaryutils::binary_stream::BinaryStream;
use std::ops::{Deref, DerefMut};
use crate::protocol::message_identifiers::ID_UNDEFINED;

pub struct Packet {
	parent : BinaryStream,
	send_time : f32,
	id : u8
}

impl Packet {
	pub fn new(buffer : Vec<u8>, offset : usize, id : u8) -> Self {
		return Self {
			parent : BinaryStream::new(buffer, offset),
			send_time : -1 as f32,
			id
		}
	}
	pub fn get_string(&mut self) -> String {
		let size : usize = self.get_short(Big) as usize;
		return String::from_utf8(self.get(size)).unwrap();
	}
	pub fn get_address(&mut self) -> InternetAddress {
		let version : u8 = self.get_byte();
		let mut addr : String;
		if version == 4 {
			addr = String::new();
			for _i in 0..3 {
				fmt(&mut addr, self.get_byte()).unwrap();
				addr.push('.');
			}
			fmt(&mut addr, self.get_byte()).unwrap();
			let port: u16 = self.get_unsigned_short(Big); // DIFF
			return InternetAddress::new(addr, port, version);
		} /*
		TODO ipv6 : need inet_ntop
		else if version == 6 {
			self.get_short(Little); //Family, AF_INET6
			let port = self.get_short(Big);
			self.get_int(Big); //flow info
		}
		*/
		else {
			panic!("BinaryDataException : Unknown IPAddress version {}", version);
		}
	}
	pub fn put_string(&mut self, v : String) {
		self.put_short(v.len() as i16, Big);
		self.put(Vec::from(v.as_str()));
	}
	pub fn put_address(&mut self, address : InternetAddress) {
		self.put_byte(address.get_version());
		if address.get_version() == 4 {
			let mut parts : Vec<u8> = Vec::new();
			for i in address.get_ip().split('.') {
				parts.push(atoi::<u8>((i as &str).as_bytes()).unwrap());
			}
			assert!(parts.len() == 4, "Wrong number of parts in IPv4, expected 4, got {}", parts.len());
		}
		/* TODO ipv6 */
		else {
			panic!("InvalidArgumentException : IP version {} is not supported", address.get_version());
		}
	}
	pub fn clean(&mut self) {
		self.reset();
		self.send_time = 0 as f32;
	}
}

impl Deref for Packet {
	type Target = BinaryStream;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for Packet {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}

impl Encode for Packet {
	const PACKET_ID: u8 = ID_UNDEFINED;
	fn decode(&mut self) {
		self.set_offset(0);
	}

	fn encode_clean(&mut self) {
		self.reset();
	}

	fn decode_clean(&mut self) {
		self.set_offset(0);
	}

	fn encode_header(&mut self) {
		let v : u8 = self.id;
		self.put_byte(v);
	}

	fn decode_header(&mut self) {
		self.get_byte();
	}
}

pub trait Encode {
	const PACKET_ID : u8;
	fn encode(&mut self) {
		self.decode_clean();
		self.encode_header();
		self.encode_payload();
	}
	fn decode(&mut self) {
		self.decode_clean();
		self.decode_header();
		self.decode_payload();
	}
	fn encode_clean(&mut self);
	fn decode_clean(&mut self);
	fn encode_header(&mut self);
	fn encode_payload(&mut self) {}
	fn decode_header(&mut self);
	fn decode_payload(&mut self) {}
}