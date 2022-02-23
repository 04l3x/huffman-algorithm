# Huffman algorithm

this is a example implementation of huffman algorithm

## usage

add to Cargo.toml
```toml
[dependencies.huffman-algorithm]
git = { "https://gitlab.com/0al3x/huffman-algorithm.git" }
```
or 

```toml
[dependencies.huffman-algorithm]
git = { "https://github.com/0al3x/huffman-algorithm.git" }
```

compression and decompression

```rust
use huffman-algorithm::{Encoder, Decoder};

fn main() {
	let input = "some input to compress";

	//compression
	let compressed = Encoder::encode(input.chars());
	println!("Input compressed:\n{:?}", compressed.encoded);

	//decompression
	let decompressed  = Decoder::decode(compressed);
	println!("Now is decompressed:\n{:?}", decompressed);
}

```

## improvements

this code can be improved by

* adding I/O module to compress/decompress from/to files
* compressing symbols table of the output, if the input is small probably the output can be more larger than input due to symbols table needed to decompression
* change map for tree in decompresssion, although probably isn't difference due to hashmap cost of get is ~O(1)
