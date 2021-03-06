use crate::protocol::packet::{Packet, Encode};
use crate::protocol::message_identifiers::ID_ADVERTISE_SYSTEM;
use std::ops::{Deref, DerefMut};

pub struct AdvertiseSystem {
	parent : Packet,
	pub server_name : String
}

impl AdvertiseSystem {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : Packet::new(buffer, offset, Self::PACKET_ID),
			server_name : String::new()
		};
	}
}

impl Deref for AdvertiseSystem {
	type Target = Packet;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}

impl DerefMut for AdvertiseSystem {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}
impl Encode for AdvertiseSystem {
	const PACKET_ID: u8 = ID_ADVERTISE_SYSTEM;

	fn encode_clean(&mut self) {
		self.parent.encode_clean();
	}
	fn decode_clean(&mut self) {
		self.parent.decode_clean();
	}
	fn encode_header(&mut self) {
		self.parent.encode_header();
	}

	fn encode_payload(&mut self) {
		let v : String = String::from(&self.server_name);
		self.put_string(v);
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.server_name = String::from(self.get_string());
	}
}