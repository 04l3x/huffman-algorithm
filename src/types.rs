use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub struct Encoded {
	pub encoded: Vec<u8>,
	pub codes: HashMap<char, Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
	pub weight: usize,
	pub left: Option<Box<Node>>,
	pub right: Option<Box<Node>>,
	pub letter: Option<char>,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		other.weight.cmp(&self.weight)
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

/* TODO
 * impl From<HashMap<char, Vec<u8>>> for Node {
	fn from(table: HashMap<char, Vec<u8>>) -> Self {
		todo!();
	}
}*/

impl From<&mut BinaryHeap<Node>> for Node {
	fn from(queue: &mut BinaryHeap<Node>) -> Self {
		while queue.len() > 1 {
			let n = queue.pop().unwrap().join(queue.pop().unwrap());
			queue.push(n)
		}

		queue.pop().unwrap()
	}
}

impl Node {
	pub fn new_leaf(weight: usize, letter: char) -> Node {
		Node {
			weight,
			left: None,
			right: None,
			letter: Some(letter),
		}
	}

	pub fn new_node(weight: usize) -> Node {
		Node {
			weight,
			left: None,
			right: None,
			letter: None,
		}
	}

	pub fn join(self, node: Node) -> Node {
		let mut parent = Node::new_node(self.weight + node.weight);
		if self.weight < node.weight {
			parent.left = Some(Box::new(self));
			parent.right = Some(Box::new(node));
		} else {
			parent.right = Some(Box::new(self));
			parent.left = Some(Box::new(node));
		}
		parent
	}

	pub fn is_leaf(&self) -> bool {
		self.left == None && self.right == None
	}
}
