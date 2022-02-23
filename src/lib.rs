mod decoder;
mod encoder;
mod types;

pub use decoder::Decoder;
pub use encoder::Encoder;

#[cfg(test)]
mod tests {
	use crate::{decoder::Decoder, encoder::Encoder};

	#[test]
	fn compress_and_decompress() {
		let input = "input to encode with huffman algorithm";

		assert_eq!(input, Decoder::decode(Encoder::encode(input.chars())));
	}
}
