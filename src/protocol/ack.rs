use crate::protocol::acknowledge_packet::AcknowledgePacket;
use crate::protocol::packet::Encode;
use std::ops::{Deref, DerefMut};

pub struct ACK {
	parent : AcknowledgePacket,
}

impl ACK {
	pub fn new(buffer : Vec<u8>, offset : usize) -> Self {
		return Self {
			parent : AcknowledgePacket::new(buffer, offset, Self::PACKET_ID)
		};
	}
}
impl Deref for ACK {
	type Target = AcknowledgePacket;

	fn deref(&self) -> &Self::Target {
		return &self.parent;
	}
}
impl DerefMut for ACK {
	fn deref_mut(&mut self) -> &mut Self::Target {
		return &mut self.parent;
	}
}

impl Encode for ACK {
	const PACKET_ID : u8 = 0xc0;

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
		self.parent.encode_payload();
	}

	fn decode_header(&mut self) {
		self.parent.decode_header();
	}

	fn decode_payload(&mut self) {
		self.parent.decode_payload();
	}
}