/* create "minimal" definition with positive+negative prefixes */
use super::super::bitstrings::BitString;
use super::{RadixSet,Node};
use std::fmt;

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Definition<S: BitString> {
	pub prefix: S,
	pub include: bool,
}

impl<S: BitString> Definition<S> {
	/// Returns a minimal list of definitions (i.e. (sub-)ranges to
	/// include and to exclude)
	///
	/// The first element of the tuple assumes branch.key[0..from_len]
	/// is excluded by the parent element, and will therefore start with
	/// a "positive" (including) definition.  The second element assumes
	/// branch.key[0..from_len] is already included by the parent
	/// element, and will start with a "negative" (excluding)
	/// definition.
	///
	/// The second element is an empty iff the branch is a single node
	/// and `from_len == branch.key().len()`.
	fn compress_branch(from_len: usize, branch: &Node<S>) -> (Vec<Definition<S>>, Vec<Definition<S>>) {
		debug_assert!(branch.key().len() >= from_len);
		match branch.key().len() - from_len {
			0 => Self::compress_inner(branch),
			pathlen => {
				let (pos, mut neg) = Self::compress_inner(branch);
				let mut excl_def = Definition{
					prefix: branch.key().clone(),
					include: false,
				};
				if 1 == pathlen && pos.len() >= neg.len() {
					// the next branch would have the same length, but
					// would exclude a higher level than necessary -
					// keep excludes longer if possible
					excl_def.prefix.flip(from_len);
					if branch.key().get(from_len) {
						neg.insert(0, excl_def);
					} else {
						neg.push(excl_def);
					}
				} else {
					excl_def.prefix.clip(from_len);
					if pos.len() < neg.len() + 2 {
						neg.clear();
						neg.push(excl_def);
						neg.extend_from_slice(&pos);
					} else {
						neg.insert(0, excl_def);
						neg.insert(1, Definition{
							prefix: branch.key().clone(),
							include: true,
						});
					}
				}
				(pos, neg)
			},
		}
	}

	/// Same as `compress_branch` but for the special case `from_len =
	/// branch.key().len()`
	fn compress_inner(n: &Node<S>) -> (Vec<Definition<S>>, Vec<Definition<S>>) {
		match *n {
			Node::Leaf(ref leaf) => {
				(vec!(Definition{
					prefix: leaf.key().clone(),
					include: true,
				}), vec!())
			},
			Node::InnerNode(ref inner) => {
				let from_len = inner.key().len() + 1;
				let (mut l_pos, mut l_neg) = Self::compress_branch(from_len, inner.left());
				let (mut r_pos, mut r_neg) = Self::compress_branch(from_len, inner.right());
				match (l_pos.len() + r_pos.len()) as isize - (l_neg.len() + r_neg.len()) as isize {
					0 | 1 | -1 => {
						l_pos.append(&mut r_pos);
						l_neg.append(&mut r_neg);
						(l_pos, l_neg)
					},
					n if n < 0 => {
						// negative list is too long
						l_pos.append(&mut r_pos);
						l_neg.clear();
						l_neg.push(Definition{
							prefix: inner.key().clone(),
							include: false,
						});
						l_neg.extend_from_slice(&l_pos[..]);
						(l_pos, l_neg)
					},
					_ => {
						// positive list is too long
						l_neg.append(&mut r_neg);
						l_pos.clear();
						l_pos.push(Definition{
							prefix: inner.key().clone(),
							include: true,
						});
						l_pos.extend_from_slice(&l_neg[..]);
						(l_pos, l_neg)
					},
				}
			},
		}
	}

	pub fn compress(s: &RadixSet<S>, invert: bool) -> Vec<Definition<S>> {
		if invert {
			match s.root() {
				Some(ref n) => {
					let mut neg = Self::compress_branch(0, n).1;
					assert!(neg.is_empty() || !neg[0].include);
					for def in &mut neg {
						def.include = !def.include;
					}
					neg
				},
				None => vec!(Definition{
					prefix: S::null(),
					include: true,
				}),
			}
		} else {
			match s.root() {
				Some(ref n) => Self::compress_branch(0, n).0,
				None => vec!(),
			}
		}
	}

	fn complete_branch_helper(from_len: usize, branch: &Node<S>, branch_key: &S, list: &mut Vec<Definition<S>>, invert: bool) {
		if from_len == branch_key.len() {
			Self::complete_inner(branch, list, invert);
		} else {
			// use recursion to produce sorted list of definitions
			let mut excl_def = Definition{
				prefix: branch_key.clone(),
				include: invert,
			};
			excl_def.prefix.flip(from_len);
			excl_def.prefix.clip(from_len + 1);

			if branch_key.get(from_len) {
				list.push(excl_def);
				Self::complete_branch(from_len + 1, branch, list, invert);
			} else {
				Self::complete_branch(from_len + 1, branch, list, invert);
				list.push(excl_def);
			}
		}
	}

	fn complete_branch(from_len: usize, branch: &Node<S>, list: &mut Vec<Definition<S>>, invert: bool) {
		Self::complete_branch_helper(from_len, branch, branch.key(), list, invert);
	}

	fn complete_inner(n: &Node<S>, list: &mut Vec<Definition<S>>, invert: bool) {
		match *n {
			Node::Leaf(ref leaf) => {
				list.push(Definition{
					prefix: leaf.key().clone(),
					include: !invert,
				});
			},
			Node::InnerNode(ref inner) => {
				let from_len = inner.key().len()+1;
				Self::complete_branch(from_len, inner.left(), list, invert);
				Self::complete_branch(from_len, inner.right(), list, invert);
			},
		}
	}

	pub fn complete(s: &RadixSet<S>, invert: bool) -> Vec<Definition<S>> {
		match s.root() {
			Some(ref n) => {
				let mut list : Vec<Definition<S>> = vec!();
				Self::complete_branch(0, n, &mut list, invert);
				list
			},
			None => vec!(Definition{
				prefix: S::null(),
				include: invert,
			}),
		}
	}
}

impl<S: BitString+fmt::Debug> fmt::Debug for Definition<S> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.include {
			write!(f, "include {:?}", self.prefix)
		} else {
			write!(f, "exclude {:?}", self.prefix)
		}
	}
}

impl<S: BitString+fmt::Display> fmt::Display for Definition<S> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.include {
			write!(f, "include {}", self.prefix)
		} else {
			write!(f, "exclude {}", self.prefix)
		}
	}
}
