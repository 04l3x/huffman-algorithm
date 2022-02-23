use crate::types::*;
use std::collections::{BinaryHeap, HashMap};

pub struct Encoder;

impl Encoder {
	fn frequencies<T>(input: T) -> HashMap<char, usize>
	where
		T: Iterator<Item = char>,
	{
		let mut freqs = HashMap::<char, usize>::new();

		for c in input {
			let count = freqs.entry(c).or_insert(0);
			*count += 1;
		}

		freqs
	}

	fn queue(input: HashMap<char, usize>) -> BinaryHeap<Node> {
		let mut queue = BinaryHeap::<Node>::new();

		for (key, val) in input.iter() {
			queue.push(Node::new_leaf(*val, *key));
		}

		queue
	}

	fn codes_table(tree: &Node, pre: Vec<u8>, codes: &mut HashMap<char, Vec<u8>>) {
		if tree.is_leaf() {
			codes.insert(tree.letter.unwrap(), pre);
		} else {
			if let Some(node) = &tree.left {
				let mut left_pre = pre.clone();
				left_pre.push(0);
				Encoder::codes_table(&node, left_pre, codes);
			}

			if let Some(node) = &tree.right {
				let mut right_pre = pre.clone();
				right_pre.push(1);
				Encoder::codes_table(&node, right_pre, codes);
			}
		}
	}

	pub fn encode<T>(input: T) -> Encoded
	where
		T: Iterator<Item = char> + Clone,
	{
		let mut codes = HashMap::new();

		Encoder::codes_table(
			&Node::from(&mut Encoder::queue(Encoder::frequencies(input.clone()))),
			vec![],
			&mut codes,
		);

		let mut output: Vec<u8> = vec![];
		let mut bits = 0;
		let mut code = 0u8;

		for c in input {
			for bit in codes.get(&c).unwrap() {
				if bits > 7 {
					output.push(code);
					bits = 0;
					code = 0u8;
				}
				code += code + bit;
				bits += 1;
			}
		}

		if bits != 0 {
			output.push(code);
		}

		Encoded {
			encoded: output,
			codes,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn count_freq() {
		let input = "input to encode with huffman algorithm";

		let output = Encoder::frequencies(input.chars());

		assert_eq!(output[&'i'], 3);
		assert_eq!(output[&'n'], 3);
		assert_eq!(output[&'p'], 1);
		assert_eq!(output[&'u'], 2);
		assert_eq!(output[&'t'], 4);
		assert_eq!(output[&'o'], 3);
		assert_eq!(output[&' '], 5);
		assert_eq!(output[&'e'], 2);
		assert_eq!(output[&'c'], 1);
		assert_eq!(output[&'h'], 3);
		assert_eq!(output[&'d'], 1);
		assert_eq!(output[&'f'], 2);
		assert_eq!(output[&'a'], 2);
		assert_eq!(output[&'m'], 2);
		assert_eq!(output[&'r'], 1);
		assert_eq!(output[&'w'], 1);
		assert_eq!(output[&'l'], 1);
		assert_eq!(output[&'g'], 1);
	}

	#[test]
	fn compression() {
		let input = "input to encode with huffman algorithm";

		let output = Encoder::encode(input.chars());

		assert!(output.encoded.len() < std::mem::size_of_val(input));
	}
}
