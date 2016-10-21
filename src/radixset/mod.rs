use super::bitstrings::BitString;
use std::boxed::Box;
use std::option::Option;
use std::mem::swap;
use std::fmt;

pub mod def;

/*
RadixSet is a binary tree with path-shortening; leafs mark prefixes
included in the set, inner nodes have no semantic value.

If an inner node had only a single child, the paths to and from it could
be shortened - therefor all inner nodes have two children
*/

#[derive(Clone)]
pub struct RadixSet<S: BitString> {
	node: Option<Node<S>>,
}

impl<S: BitString+fmt::Debug> fmt::Debug for RadixSet<S> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.node {
			None => {
				write!(f, "RadixSet {{ }}")
			},
			Some(ref node) => {
				write!(f, "RadixSet {{ {:?} }}", node)
			},
		}
	}
}

impl<S: BitString> Default for RadixSet<S> {
	fn default() -> RadixSet<S> {
		return RadixSet::<S>{
			node: None,
		}
	}
}

#[derive(Clone)]
pub enum Node<S: BitString> {
	InnerNode(InnerNode<S>),
	Leaf(Leaf<S>),
}

#[derive(Clone)]
pub struct Leaf<S: BitString> {
	key: S,
}

#[derive(Clone)]
pub struct InnerNode<S: BitString> {
	key: S,
	children: Box<Children<S>>,
}

#[derive(Clone)]
struct Children<S: BitString> {
	left: Node<S>,
	right: Node<S>,
}

impl<S: BitString> Leaf<S> {
	pub fn key(&self) -> &S {
		&self.key
	}
}

impl<S: BitString> InnerNode<S> {
	fn pick_side<'a>(&'a mut self, subkey: &S) -> &'a mut Node<S> {
		if subkey.get(self.key.len()) {
			&mut self.children.right
		} else {
			&mut self.children.left
		}
	}

	pub fn key(&self) -> &S {
		&self.key
	}

	pub fn left(&self) -> &Node<S> {
		&self.children.left
	}

	pub fn right(&self) -> &Node<S> {
		&self.children.right
	}
}

impl<S: BitString+fmt::Debug> fmt::Debug for Node<S> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Node::Leaf(ref leaf) => write!(f, "Leaf {{ key: {:?} }}", leaf.key),
			Node::InnerNode(ref inner) => write!(f, "InnerNode {{ key: {:?}, left: {:?}, right: {:?} }}", inner.key, inner.children.left, inner.children.right),
		}
	}
}

impl<S: BitString> Node<S> {
	fn new_leaf(key: &S) -> Node<S> {
		Node::Leaf(Leaf{
			key: key.clone(),
		})
	}

	fn new_inner(key: &S, left: Node<S>, right: Node<S>) -> Node<S> {
		Node::InnerNode(InnerNode{
			key: key.clone(),
			children: Box::new(Children{
				left: left,
				right: right,
			}),
		})
	}

	pub fn key(&self) -> &S {
		match *self {
			Node::Leaf(ref leaf) => &leaf.key,
			Node::InnerNode(ref inner) => &inner.key,
		}
	}

	fn key_mut_ref(&mut self) -> &mut S {
		match *self {
			Node::Leaf(ref mut leaf) => &mut leaf.key,
			Node::InnerNode(ref mut inner) => &mut inner.key,
		}
	}

	fn convert_leaf(&mut self) {
		let leaf = Self::new_leaf(match *self {
			Node::Leaf(_) => return,
			Node::InnerNode(ref inner) => &inner.key,
		});
		*self = leaf;
	}

	fn insert_children(&mut self, a: Node<S>, b: Node<S>) {
		let inner = {
			let key = match *self {
				Node::Leaf(ref leaf) => &leaf.key,
				_ => panic!("Cannot add children to inner node"),
			};
			let key_len = key.len();
			let a_right = a.key().get(key_len);
			assert_eq!(!a_right, b.key().get(key_len));
			if a_right {
				Self::new_inner(key, b, a)
			} else {
				Self::new_inner(key, a, b)
			}
		};
		*self = inner;
	}

	pub fn insert(&mut self, key: &S) {
		let (mut self_key_len, shared_prefix_len) = {
			let key_ref = self.key();
			(key_ref.len(), key_ref.shared_prefix_len(key))
		};
		if shared_prefix_len == self_key_len {
			// new key same as self.key or below in tree
			match *self {
				Node::Leaf(_) => return, // -> already included
				_ if shared_prefix_len == key.len() => {
					// keys equal: make current node a leaf node
					self.convert_leaf();
					return; // no need to compress below
				},
				Node::InnerNode(ref mut inner) => inner.pick_side(key).insert(key),
			}
		} else if shared_prefix_len == key.len() {
			// new key is parent of self: clip key and make node a leaf node
			self.key_mut_ref().clip(shared_prefix_len);
			self.convert_leaf();
			return; // no need to compress below
		} else {
			// need to create separate parent node for new key and self
			let mut tmp_node = Self::new_leaf(&self.key());
			tmp_node.key_mut_ref().clip(shared_prefix_len);
			swap(self, &mut tmp_node);
			self.insert_children(tmp_node, Self::new_leaf(key));
			// update self_key_len for compression handling below
			self_key_len = shared_prefix_len;
		}

		// compress: if node has two children, and both sub keys
		// are exactly one bit longer than the key of the parent
		// node, and both child nodes are leafs, make the
		// current node a leaf
		let compress = match *self {
			Node::InnerNode(ref inner) => {
				let compress_left = match inner.children.left {
					Node::Leaf(ref left_leaf) => left_leaf.key.len() == self_key_len + 1,
					Node::InnerNode(_) => return, // must be leaf
				};
				let compress_right = match inner.children.right {
					Node::Leaf(ref right_leaf) => right_leaf.key.len() == self_key_len + 1,
					Node::InnerNode(_) => return, // must be leaf
				};
				compress_left && compress_right
			},
			Node::Leaf(_) => return, // already compressed
		};
		if compress {
			self.convert_leaf();
		}
	}
}

impl<S: BitString> RadixSet<S> {
	pub fn insert(&mut self, key: &S) {
		match self.node {
			None => {
				self.node = Some(Node::new_leaf(key));
			},
			Some(ref mut node) => {
				node.insert(key);
			},
		}
	}

	pub fn root(&self) -> Option<&Node<S>> {
		match self.node {
			None => None,
			Some(ref node) => Some(&node),
		}
	}
}
