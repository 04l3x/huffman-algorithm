use crate::types::*;
use std::collections::HashMap;

pub struct Decoder;

impl Decoder {
	fn to_bits(encoded: Vec<u8>) -> Vec<u8> {
		let mut output: Vec<u8> = vec![];

		for mut byte in encoded {
			let mut bits: Vec<u8> = vec![];

			while byte != 0 {
				bits.push(byte % 2);
				byte /= 2;
			}

			if bits.len() < 8 {
				bits.append(&mut vec![0u8; 8 - bits.len()])
			}

			bits.reverse();

			output.append(&mut bits);
		}

		output
	}

	pub fn decode(input: Encoded) -> String {
		let mut codes: HashMap<Vec<u8>, char> = HashMap::new();

		for (key, value) in input.codes {
			codes.insert(value, key);
		}

		let mut output = String::new();
		let mut buffer: Vec<u8> = vec![];
		for bit in Decoder::to_bits(input.encoded) {
			buffer.push(bit);

			//FIXME?: change table for tree
			//NOTE: probably not really diference betwen try gettin code from map or checking node tree
			match codes.get(&buffer) {
				Some(c) => {
					output.push(*c);
					buffer = vec![];
				}
				_ => {}
			}
		}

		output
	}
}
